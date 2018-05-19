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

    'windows': {},
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
            code = int(tokens[3][0].group(0))
            if code <= 0:
                raise RuntimeError(
                    '%s:%d: Errno code <= 0 (%d)' % (
                        f_name, line_n, code))
        elif tokens[3][1] == _T_IDENTIFIER:
            existing_ident = tokens[3][0].group(0)
            existing = errno_consts.get(existing_ident)
            if existing is None:
                raise RuntimeError(
                    '%s:%d: Existing errno constant not found (%s)' % (
                        f_name, line_n, existing_ident))
            else:
                code = existing_ident
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
            raise RuntimeError(
                '%s:%d: No comments' % (
                    f_name, line_n))
        if len(comments) > 1:
            raise RuntimeError(
                '%s:%d: Too many comments' % (
                    f_name, line_n))

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
// For code generated by `phf_codegen`.
#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]

use std;

use phf;

use ErrnoCode;
use ErrnoCodes;

'''.lstrip('\n')
_ENDING = '''
include!(concat!(env!("OUT_DIR"), "/{f_name}"));
'''.lstrip('\n')

_ERRNO_CODE = '''
/// {comment_with_dot}
pub const {identifier}: std::os::raw::c_int = {code};
/// Human-readable message for {identifier}.
pub const {identifier}_MSG: &str = "{comment}";
/// Identifier for {identifier} as a string (equals to "{identifier}").
pub const {identifier}_ID: &str = "{identifier}";
'''.lstrip('\n')

_CODEGEN_PREAMBLE = '''
phf_codegen::Map::new()
'''.lstrip('\n')
_CODEGEN_BY_ID = '''
    .entry("{identifier}", "ErrnoCode {{
        num: {identifier},
        msg: {identifier}_MSG,
        id: {identifier}_ID,
    }}")
'''.lstrip('\n')
_CODEGEN_BY_NUM_BEGIN = '''
    .entry({num}, "&[
'''.lstrip('\n')
_CODEGEN_BY_NUM_ELEM = '''
        ErrnoCode {{
            num: {identifier},
            msg: {identifier}_MSG,
            id: {identifier}_ID,
        }},
'''.lstrip('\n')
_CODEGEN_BY_NUM_END = '''
    ]")
'''.lstrip('\n')


def _generate(token_lines, quirks, include_f_name, codegen_f_name_base):
    errno_consts = _get_errno_consts(token_lines, quirks)

    by_num = {}

    sys.stdout.write(_PREAMBLE + '\n')
    for identifier, info in six.iteritems(errno_consts):
        comment = info['comment'].rstrip('.')
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

        sys.stdout.write(_ERRNO_CODE.format(**info) + '\n')

    sys.stdout.write(_ENDING.format(f_name=include_f_name))
    sys.stdout.flush()

    with open(codegen_f_name_base + '-codegen-by-id.rs', 'w') as codegen:
        codegen.write(_CODEGEN_PREAMBLE)
        for identifier in six.iterkeys(errno_consts):
            codegen.write(_CODEGEN_BY_ID.format(identifier=identifier))

    with open(codegen_f_name_base + '-codegen-by-num.rs', 'w') as codegen:
        codegen.write(_CODEGEN_PREAMBLE)
        for num, identifier_list in six.iteritems(by_num):
            codegen.write(_CODEGEN_BY_NUM_BEGIN.format(num=num))
            for identifier in identifier_list:
                codegen.write(
                    _CODEGEN_BY_NUM_ELEM.format(identifier=identifier))
            codegen.write(_CODEGEN_BY_NUM_END)


def _main():
    cmd = sys.argv[1]
    f_name = sys.argv[2]
    quirks = _QUIRKS[sys.argv[3]]
    if cmd == 'test':
        include_f_name = None
        codegen_f_name_base = None
        include_paths = sys.argv[4:]
    else:
        include_f_name = sys.argv[4]
        codegen_f_name_base = sys.argv[5]
        include_paths = sys.argv[6:]

    token_lines = _read_tokens_from_file(f_name, include_paths, quirks)

    if cmd == 'test':
        _test(token_lines, quirks)
    elif cmd == 'generate':
        _generate(token_lines, quirks, include_f_name, codegen_f_name_base)
    else:
        raise RuntimeError(
            'Unknown command: %r' % (
                cmd, ))


_main()
