#!/usr/bin/env python

# Contains very primitive and incomplete parser of C preprocessor directives.
# May accidentally read invalid C source without any errors.

import collections
import json
import os.path
import re
import sys

import six


_TOKENS = (
    _T_HASH,
    _T_IDENTIFIER,
    _T_INT,
    _T_STRING,
    _T_INCLUDE,
    _T_OTHER,
) = (
    re.compile(r'#'),
    re.compile(r'[_a-zA-Z][_0-9a-zA-Z]*'),
    re.compile(r'[+-]?[0-9]+'),
    re.compile(r'"([^"]*)"'),  # TODO: "a\"b".
    re.compile(r'<([^>]*)>'),
    re.compile(r'\S+'),
)

_QUIRKS = {
    'none': {},

    'linux': {},

    'macos': {
        'no_includes': True,
        'id_blacklist': {'errno'},
    },

    'windows': {
        'no_includes': True,
        'id_blacklist': {'errno'},
    },
}


def _read_lines(f_name):
    line_n = 0
    rel_f_name = os.path.relpath(f_name)
    with open(f_name, 'r') as f_obj:
        for raw_line in f_obj:
            line_n += 1
            yield rel_f_name, line_n, raw_line


def _merge_continued_lines(raw_lines):
    f_name = '<error>'
    line_n = -1

    prev = None
    for f_name, line_n, raw_line in raw_lines:
        rstripped = raw_line.rstrip()
        if rstripped[-1:] == '\\':
            if prev is None:
                prev = rstripped[:-1]
            else:
                prev += rstripped[:-1]
        else:
            if prev is not None:
                raw_line = prev + raw_line
                prev = None

            yield f_name, line_n, raw_line

    if prev is not None:
        raise RuntimeError(
            '%s:%d: EOF after backslash-newline' % (
                f_name, line_n))


def _replace_comments(raw_lines):
    # TODO: Ignore comments within string literals.
    f_name = '<error>'
    line_n = -1

    multiline_block = None
    prev = None
    for f_name, line_n, raw_line in raw_lines:
        ready_comments = []

        if multiline_block is not None:
            # Multiline block comment was previously started.

            block_end = raw_line.find('*/')
            if block_end == -1:
                # Whole line is in the block comment.
                multiline_block += raw_line
                raw_line = ''
            else:
                # End of the block comment.
                multiline_block += raw_line[:block_end]
                raw_line = prev + ' ' + raw_line[block_end + 2:]
                prev = None
                ready_comments.append(multiline_block)
                multiline_block = None

        if multiline_block is None:
            # Normal line.

            search_pos = 0
            while search_pos < len(raw_line):
                block_start = raw_line.find('/*', search_pos)
                if block_start == -1:
                    # No block comments in this line.
                    new_multiline_block = None
                    block_comment = None
                    new_raw_line = raw_line
                    new_search_pos = len(new_raw_line)
                else:
                    block_end = raw_line.find('*/', block_start + 2)
                    if block_end == -1:
                        # Start of multiline block comment.
                        new_multiline_block = raw_line[block_start + 2:]
                        block_comment = None
                        new_raw_line = ''
                        prev = raw_line[:block_start]
                        new_search_pos = len(new_raw_line)
                    else:
                        # Short block comment.
                        new_multiline_block = None
                        block_comment = raw_line[block_start + 2:block_end]
                        new_raw_line = \
                            raw_line[:block_start] + \
                            ' ' + \
                            raw_line[block_end + 2:]
                        new_search_pos = block_start + 1

                # Check for line comment.
                if block_start == -1:
                    block_start = len(raw_line)
                line_start = raw_line.find('//', search_pos, block_start)
                if line_start == -1:
                    multiline_block = new_multiline_block

                    if block_comment is not None:
                        ready_comments.append(block_comment)

                    raw_line = new_raw_line
                    search_pos = new_search_pos
                else:
                    prev = None
                    ready_comments.append(raw_line[line_start + 2:])
                    raw_line = raw_line[:line_start]
                    search_pos = len(raw_line)

        yield f_name, line_n, raw_line, ready_comments

    if multiline_block is not None:
        raise RuntimeError(
            '%s:%d: EOF without closing multiline block comment' % (
                f_name, line_n))


def _skip_empty(raw_lines):
    for f_name, line_n, raw_line, comments in raw_lines:
        if raw_line.strip():
            yield f_name, line_n, raw_line, comments


def _tokenize(raw_lines):
    for f_name, line_n, raw_line, comments in raw_lines:
        tokens = []

        while True:
            raw_line = raw_line.lstrip()
            if not raw_line:
                break

            match = None
            match_re = None
            for token_re in _TOKENS:
                match = token_re.match(raw_line)
                if match is not None:
                    match_re = token_re
                    break

            if match is None:
                raise RuntimeError(
                    '%s:%d: Unknown token %r' % (
                        f_name, line_n, raw_line))
            else:
                raw_line = raw_line[len(match.group(0)):]
                tokens.append((match, match_re))

        yield f_name, line_n, tokens, comments


def _match_token(token, reference):
    if reference is None:
        # Any token.
        return True
    if hasattr(reference, 'match'):
        # Matched with specific regex.
        _, match_re = token
        return match_re == reference

    # Fixed string.
    match, _ = token
    return match.group(0) == reference


def _match_tokens(tokens, reference, full=True):
    if reference:
        if tokens:
            return _match_token(tokens[0], reference[0]) & \
                   _match_tokens(tokens[1:], reference[1:], full)
        else:
            return False
    else:
        if full:
            if tokens:
                return False
            else:
                return True
        else:
            return True


def _process_ifdefs(token_lines, defined=None):
    # TODO: support #if and #elif.
    f_name = '<error>'
    line_n = -1

    if defined is None:
        defined = {}

    state = []
    enabled = []
    for f_name, line_n, tokens, comments in token_lines:
        if _match_tokens(
                tokens,
                (
                        _T_HASH,
                        'ifdef',
                        _T_IDENTIFIER,
                )):
            state.append('ifdef')
            enabled.append(tokens[2][0].group(0) in defined)
        elif _match_tokens(
                tokens,
                (
                        _T_HASH,
                        'ifndef',
                        _T_IDENTIFIER,
                )):
            state.append('ifndef')
            enabled.append(tokens[2][0].group(0) not in defined)
        elif _match_tokens(
                tokens,
                (
                        _T_HASH,
                        'else',
                )):
            if not state:
                raise RuntimeError(
                    '%s:%d: Unexpected #else (no #ifdef/#ifndef)' % (
                        f_name, line_n))
            if state.pop() in ('ifdef', 'ifndef'):
                state.append('else')
                enabled[-1] = not enabled[-1]
            else:
                raise RuntimeError(
                    '%s:%d: Unexpected #else (not #ifdef/#ifndef)' % (
                        f_name, line_n))
        elif _match_tokens(
                tokens,
                (
                        _T_HASH,
                        'endif',
                )):
            if not state:
                raise RuntimeError(
                    '%s:%d: Unexpected #endif (no #ifdef/#ifndef/#else)' % (
                        f_name, line_n))
            state.pop()
            enabled.pop()
        elif _match_tokens(
                tokens,
                (
                        _T_HASH,
                        'if',
                        None,
                ), False):
            # Hack to "ignore" #if.
            # Breaks #else.
            state.append('ifdef')
            enabled.append(False)
        else:
            if all(enabled):
                yield f_name, line_n, tokens, comments

    if state:
        raise RuntimeError(
            '%s:%d: #ifdef/#ifndef not closed' % (
                f_name, line_n))


def _read_tokens_from_single_file(f_name):
    lines = _read_lines(f_name)
    lines = _merge_continued_lines(lines)
    lines = _replace_comments(lines)
    lines = _skip_empty(lines)
    token_lines = _tokenize(lines)
    token_lines = _process_ifdefs(token_lines)
    return token_lines


def _find_include(base_f_name, base_line_n, inc_rel, include_paths):
    for inc_path in include_paths:
        inc_full = os.path.join(inc_path, inc_rel)
        if os.path.exists(inc_full):
            return inc_full

    raise RuntimeError(
        '%s:%d: File %r not found' % (
            base_f_name, base_line_n, inc_rel))


def _read_tokens_from_file(f_name, include_paths, quirks):
    no_includes = quirks.get('no_includes', False)

    files = [_read_tokens_from_single_file(f_name)]
    past_includes = {f_name}
    while files:
        try:
            f_name, line_n, tokens, comments = next(files[-1])
            if _match_tokens(
                    tokens,
                    (
                            _T_HASH,
                            'include',
                            None,
                    )):
                include, include_re = tokens[2]

                if not no_includes:
                    if include_re == _T_STRING:
                        cur_include_paths = \
                            [os.path.dirname(os.path.abspath(f_name))] + \
                            include_paths
                    elif include_re == _T_INCLUDE:
                        cur_include_paths = include_paths
                    else:
                        raise RuntimeError(
                            '%s:%d: Invalid include' % (
                                f_name, line_n))

                    if include.group(1) in past_includes:
                        raise RuntimeError(
                            '%s:%d: Duplicate include: %r' % (
                                f_name, line_n, include.group(1)))
                    past_includes.add(include.group(1))

                    include = _find_include(f_name, line_n, include.group(1),
                                            cur_include_paths)
                    files.append(_read_tokens_from_single_file(include))
            else:
                yield f_name, line_n, tokens, comments
        except StopIteration:
            files.pop()


def _get_errno_consts(token_lines, quirks):
    errno_consts = collections.OrderedDict()
    # Saves only first code.
    by_num = {}

    id_blacklist = quirks.get('id_blacklist', {})

    for f_name, line_n, tokens, comments in token_lines:
        if _match_tokens(
                tokens,
                (
                        _T_HASH,
                        'undef',
                        _T_IDENTIFIER,
                ), False):
            ident = tokens[2][0].group(0)
            if ident[:1] == 'E':
                if ident not in errno_consts:
                    raise RuntimeError(
                        '%s:%d: #undef of undefined macro' % (
                            f_name, line_n))

                errno_consts.pop(ident)
            continue
        elif not _match_tokens(
                tokens,
                (
                       _T_HASH,
                        'define',
                        None,
                        None,
                ), False):
            continue
        if len(tokens) != 4:
            raise RuntimeError(
                '%s:%d: Unexpected number of tokens' % (
                    f_name, line_n))

        if tokens[2][1] != _T_IDENTIFIER:
            raise RuntimeError(
                '%s:%d: Invalid identifier' % (
                    f_name, line_n))
        ident = tokens[2][0].group(0)

        if ident in id_blacklist:
            continue

        if len(ident) < 3:
            raise RuntimeError(
                '%s:%d: Too short identifier' % (
                    f_name, line_n))
        if ident.upper() != ident:
            raise RuntimeError(
                '%s:%d: Identifier contains non-capital letters' % (
                    f_name, line_n))
        if ident[0] != 'E':
            raise RuntimeError(
                '%s:%d: Identifier is not starting with \'E\'' % (
                    f_name, line_n))

        if tokens[3][1] == _T_INT:
            num = int(tokens[3][0].group(0))
            if num <= 0:
                raise RuntimeError(
                    '%s:%d: Errno code <= 0 (%d)' % (
                        f_name, line_n, num))
            code = num
        elif tokens[3][1] == _T_IDENTIFIER:
            existing_ident = tokens[3][0].group(0)
            existing = errno_consts.get(existing_ident)
            if existing is None:
                raise RuntimeError(
                    '%s:%d: Existing errno constant not found (%s)' % (
                        f_name, line_n, existing_ident))
            else:
                code = existing_ident

                num = existing_ident
                while isinstance(num, six.string_types):
                    num = errno_consts[num]['code']

                if not comments:
                    comments = ['Same as %s (%s)' % (existing_ident,
                                                     existing['comment'])]
        else:
            raise RuntimeError(
                '%s:%d: Invalid errno code' % (
                    f_name, line_n))

        if ident in errno_consts:
            raise RuntimeError(
                '%s:%d: Duplicate definition (%s)' % (
                    f_name, line_n, ident))

        if not comments:
            existing_ident = by_num.get(num)
            if existing_ident is None:
                raise RuntimeError(
                    '%s:%d: No comments' % (
                        f_name, line_n))
            else:
                existing = errno_consts[existing_ident]
                comments = ['Same as %s (%s)' % (existing_ident,
                                                 existing['comment'])]

        if len(comments) > 1:
            raise RuntimeError(
                '%s:%d: Too many comments' % (
                    f_name, line_n))

        by_num.setdefault(num, ident)

        errno_consts[ident] = {
            'code': code,
            'comment': comments[0].strip()
        }

    return errno_consts


def _test(token_lines, quirks):
    result = {
        'tokens': [],
    }

    def _append_tokens(tl):
        for f, l, t, c in tl:
            result['tokens'].append(
                [
                    list(map(lambda m: m[0].group(0), t)),
                    c,
                ]
            )
            yield f, l, t, c

    try:
        result['errno_consts'] = _get_errno_consts(
            _append_tokens(token_lines), quirks)
    except RuntimeError as error:
        sys.stdout.write('ERROR: %s\n' % (error.args[0], ))
        sys.stdout.flush()
        sys.exit(1)

    json.dump(result, sys.stdout, indent=4, separators=(',', ': '),
              sort_keys=True)
    sys.stdout.write('\n')
    sys.stdout.flush()


_PREAMBLE = '''
// This file is generated automatically. Do not modify it by hand.

// For code generated by `phf_codegen`.
#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]

use std;

use phf;

use ErrnoCode;
use ErrnoCodes;

/// Return string describing error number.
///
/// Returns None on unknown errnum.
///
/// Returns only the first message if there are multiple error messages defined
/// for the same numeric constant.
pub fn strerror(errnum: std::os::raw::c_int) -> Option<&'static str> {
    BY_NUM.get(&errnum).map(|code| code[0].msg)
}

/// Return short string identifier for error number.
///
/// Returns None on unknown errnum.
///
/// Returns only the first identifier if there are multiple identifiers defined
/// for the same numeric constant.
pub fn strerror_id(errnum: std::os::raw::c_int) -> Option<&'static str> {
    BY_NUM.get(&errnum).map(|code| code[0].id)
}
'''.lstrip('\n')
_ENDING = '''
include!(concat!(env!("OUT_DIR"), "/{f_name}"));

#[cfg(test)]
mod tests {{
    #[test]
    fn strerror() {{
        assert_eq!(super::strerror(super::EDOM), Some(super::EDOM_MSG));
        assert_eq!(super::strerror(0), None);
    }}

    #[test]
    fn strerror_id() {{
        assert_eq!(super::strerror_id(super::EDOM), Some("EDOM"));
        assert_eq!(super::strerror_id(0), None);
    }}
}}
'''.lstrip('\n')

_ERRNO_CODE = '''
/// {comment_with_dot}
pub const {identifier}: std::os::raw::c_int = {code};
/// Human-readable message for {identifier}.
pub const {identifier}_MSG: &str = "{comment}";
/// Identifier for {identifier} as a string (equals to "{identifier}").
pub const {identifier}_ID: &str = "{identifier}";
'''.lstrip('\n')

_CODEGEN_ERRNO_CODE = '''
ErrnoCode {{ num: {identifier}, msg: {identifier}_MSG, id: {identifier}_ID }}
'''.strip()
_CODEGEN_BY_NUM_BEGIN = '&['
_CODEGEN_BY_NUM_END = ']'


def _get_include_name(base_f_name):
    rel_base = os.path.relpath(base_f_name)
    path = []
    while True:
        rel_base, tail = os.path.split(rel_base)
        if tail == 'src':
            break
        else:
            path.insert(0, tail)
    return '.'.join(path) + '.rs'


def _generate(token_lines, quirks, base_f_name):
    errno_consts = _get_errno_consts(token_lines, quirks)

    by_num = collections.OrderedDict()

    with open(base_f_name + '.rs', 'w') as f_obj:
        f_obj.write(_PREAMBLE + '\n')
        for identifier, info in six.iteritems(errno_consts):
            # TODO: Properly escape characters in comment.
            comment = info['comment'].rstrip('.').replace('"', '\\"')
            comment_with_dot = comment + '.'
            info = dict(
                info,
                identifier=identifier,
                comment=comment,
                comment_with_dot=comment_with_dot)

            code = info['code']
            if isinstance(code, six.string_types):
                code = errno_consts[code]['code']
            by_num_identifier_list = by_num.get(code, [])
            if not by_num_identifier_list:
                by_num[code] = by_num_identifier_list
            by_num_identifier_list.append(identifier)

            f_obj.write(_ERRNO_CODE.format(**info) + '\n')

        f_obj.write(_ENDING.format(f_name=_get_include_name(base_f_name)))

    codegen_json = collections.OrderedDict()
    codegen_json['_comment'] = 'This file is generated automatically. Do not ' \
                               'modify it by hand.'
    codegen_json['by_id'] = collections.OrderedDict(
        map(
            lambda ident: (ident,
                           _CODEGEN_ERRNO_CODE.format(identifier=ident)),
            six.iterkeys(errno_consts)))
    codegen_json['by_num'] = collections.OrderedDict(
        map(
            lambda kv: (
                kv[0],
                _CODEGEN_BY_NUM_BEGIN + ', '.join(
                    map(
                        lambda ident: _CODEGEN_ERRNO_CODE.format(
                            identifier=ident),
                        kv[1],
                    )) + _CODEGEN_BY_NUM_END,
            ),
            six.iteritems(by_num)))

    with open(base_f_name + '.json', 'w') as json_f_obj:
        json.dump(codegen_json, json_f_obj, indent=4, separators=(',', ': '))
        json_f_obj.write('\n')


def _main():
    cmd = sys.argv[1]
    quirks = _QUIRKS[sys.argv[2]]
    main_include_f_name = sys.argv[3]
    if cmd == 'test':
        base_f_name = None
        include_paths = sys.argv[4:]
    else:
        base_f_name = sys.argv[4]
        include_paths = sys.argv[5:]

    token_lines = _read_tokens_from_file(
        main_include_f_name, include_paths, quirks)

    if cmd == 'test':
        _test(token_lines, quirks)
    elif cmd == 'generate':
        _generate(token_lines, quirks, base_f_name)
    else:
        raise RuntimeError(
            'Unknown command: %r' % (
                cmd, ))


_main()
