// For code generated by `phf_codegen`.
#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]

use std;

use phf;

use ErrnoCode;
use ErrnoCodes;

/// Operation not permitted.
pub const EPERM: std::os::raw::c_int = 1;
/// Human-readable message for EPERM.
pub const EPERM_MSG: &str = "Operation not permitted";
/// Identifier for EPERM as a string (equals to "EPERM").
pub const EPERM_ID: &str = "EPERM";

/// No such file or directory.
pub const ENOENT: std::os::raw::c_int = 2;
/// Human-readable message for ENOENT.
pub const ENOENT_MSG: &str = "No such file or directory";
/// Identifier for ENOENT as a string (equals to "ENOENT").
pub const ENOENT_ID: &str = "ENOENT";

/// No such process.
pub const ESRCH: std::os::raw::c_int = 3;
/// Human-readable message for ESRCH.
pub const ESRCH_MSG: &str = "No such process";
/// Identifier for ESRCH as a string (equals to "ESRCH").
pub const ESRCH_ID: &str = "ESRCH";

/// Interrupted system call.
pub const EINTR: std::os::raw::c_int = 4;
/// Human-readable message for EINTR.
pub const EINTR_MSG: &str = "Interrupted system call";
/// Identifier for EINTR as a string (equals to "EINTR").
pub const EINTR_ID: &str = "EINTR";

/// I/O error.
pub const EIO: std::os::raw::c_int = 5;
/// Human-readable message for EIO.
pub const EIO_MSG: &str = "I/O error";
/// Identifier for EIO as a string (equals to "EIO").
pub const EIO_ID: &str = "EIO";

/// No such device or address.
pub const ENXIO: std::os::raw::c_int = 6;
/// Human-readable message for ENXIO.
pub const ENXIO_MSG: &str = "No such device or address";
/// Identifier for ENXIO as a string (equals to "ENXIO").
pub const ENXIO_ID: &str = "ENXIO";

/// Argument list too long.
pub const E2BIG: std::os::raw::c_int = 7;
/// Human-readable message for E2BIG.
pub const E2BIG_MSG: &str = "Argument list too long";
/// Identifier for E2BIG as a string (equals to "E2BIG").
pub const E2BIG_ID: &str = "E2BIG";

/// Exec format error.
pub const ENOEXEC: std::os::raw::c_int = 8;
/// Human-readable message for ENOEXEC.
pub const ENOEXEC_MSG: &str = "Exec format error";
/// Identifier for ENOEXEC as a string (equals to "ENOEXEC").
pub const ENOEXEC_ID: &str = "ENOEXEC";

/// Bad file number.
pub const EBADF: std::os::raw::c_int = 9;
/// Human-readable message for EBADF.
pub const EBADF_MSG: &str = "Bad file number";
/// Identifier for EBADF as a string (equals to "EBADF").
pub const EBADF_ID: &str = "EBADF";

/// No child processes.
pub const ECHILD: std::os::raw::c_int = 10;
/// Human-readable message for ECHILD.
pub const ECHILD_MSG: &str = "No child processes";
/// Identifier for ECHILD as a string (equals to "ECHILD").
pub const ECHILD_ID: &str = "ECHILD";

/// Try again.
pub const EAGAIN: std::os::raw::c_int = 11;
/// Human-readable message for EAGAIN.
pub const EAGAIN_MSG: &str = "Try again";
/// Identifier for EAGAIN as a string (equals to "EAGAIN").
pub const EAGAIN_ID: &str = "EAGAIN";

/// Out of memory.
pub const ENOMEM: std::os::raw::c_int = 12;
/// Human-readable message for ENOMEM.
pub const ENOMEM_MSG: &str = "Out of memory";
/// Identifier for ENOMEM as a string (equals to "ENOMEM").
pub const ENOMEM_ID: &str = "ENOMEM";

/// Permission denied.
pub const EACCES: std::os::raw::c_int = 13;
/// Human-readable message for EACCES.
pub const EACCES_MSG: &str = "Permission denied";
/// Identifier for EACCES as a string (equals to "EACCES").
pub const EACCES_ID: &str = "EACCES";

/// Bad address.
pub const EFAULT: std::os::raw::c_int = 14;
/// Human-readable message for EFAULT.
pub const EFAULT_MSG: &str = "Bad address";
/// Identifier for EFAULT as a string (equals to "EFAULT").
pub const EFAULT_ID: &str = "EFAULT";

/// Block device required.
pub const ENOTBLK: std::os::raw::c_int = 15;
/// Human-readable message for ENOTBLK.
pub const ENOTBLK_MSG: &str = "Block device required";
/// Identifier for ENOTBLK as a string (equals to "ENOTBLK").
pub const ENOTBLK_ID: &str = "ENOTBLK";

/// Device or resource busy.
pub const EBUSY: std::os::raw::c_int = 16;
/// Human-readable message for EBUSY.
pub const EBUSY_MSG: &str = "Device or resource busy";
/// Identifier for EBUSY as a string (equals to "EBUSY").
pub const EBUSY_ID: &str = "EBUSY";

/// File exists.
pub const EEXIST: std::os::raw::c_int = 17;
/// Human-readable message for EEXIST.
pub const EEXIST_MSG: &str = "File exists";
/// Identifier for EEXIST as a string (equals to "EEXIST").
pub const EEXIST_ID: &str = "EEXIST";

/// Cross-device link.
pub const EXDEV: std::os::raw::c_int = 18;
/// Human-readable message for EXDEV.
pub const EXDEV_MSG: &str = "Cross-device link";
/// Identifier for EXDEV as a string (equals to "EXDEV").
pub const EXDEV_ID: &str = "EXDEV";

/// No such device.
pub const ENODEV: std::os::raw::c_int = 19;
/// Human-readable message for ENODEV.
pub const ENODEV_MSG: &str = "No such device";
/// Identifier for ENODEV as a string (equals to "ENODEV").
pub const ENODEV_ID: &str = "ENODEV";

/// Not a directory.
pub const ENOTDIR: std::os::raw::c_int = 20;
/// Human-readable message for ENOTDIR.
pub const ENOTDIR_MSG: &str = "Not a directory";
/// Identifier for ENOTDIR as a string (equals to "ENOTDIR").
pub const ENOTDIR_ID: &str = "ENOTDIR";

/// Is a directory.
pub const EISDIR: std::os::raw::c_int = 21;
/// Human-readable message for EISDIR.
pub const EISDIR_MSG: &str = "Is a directory";
/// Identifier for EISDIR as a string (equals to "EISDIR").
pub const EISDIR_ID: &str = "EISDIR";

/// Invalid argument.
pub const EINVAL: std::os::raw::c_int = 22;
/// Human-readable message for EINVAL.
pub const EINVAL_MSG: &str = "Invalid argument";
/// Identifier for EINVAL as a string (equals to "EINVAL").
pub const EINVAL_ID: &str = "EINVAL";

/// File table overflow.
pub const ENFILE: std::os::raw::c_int = 23;
/// Human-readable message for ENFILE.
pub const ENFILE_MSG: &str = "File table overflow";
/// Identifier for ENFILE as a string (equals to "ENFILE").
pub const ENFILE_ID: &str = "ENFILE";

/// Too many open files.
pub const EMFILE: std::os::raw::c_int = 24;
/// Human-readable message for EMFILE.
pub const EMFILE_MSG: &str = "Too many open files";
/// Identifier for EMFILE as a string (equals to "EMFILE").
pub const EMFILE_ID: &str = "EMFILE";

/// Not a typewriter.
pub const ENOTTY: std::os::raw::c_int = 25;
/// Human-readable message for ENOTTY.
pub const ENOTTY_MSG: &str = "Not a typewriter";
/// Identifier for ENOTTY as a string (equals to "ENOTTY").
pub const ENOTTY_ID: &str = "ENOTTY";

/// Text file busy.
pub const ETXTBSY: std::os::raw::c_int = 26;
/// Human-readable message for ETXTBSY.
pub const ETXTBSY_MSG: &str = "Text file busy";
/// Identifier for ETXTBSY as a string (equals to "ETXTBSY").
pub const ETXTBSY_ID: &str = "ETXTBSY";

/// File too large.
pub const EFBIG: std::os::raw::c_int = 27;
/// Human-readable message for EFBIG.
pub const EFBIG_MSG: &str = "File too large";
/// Identifier for EFBIG as a string (equals to "EFBIG").
pub const EFBIG_ID: &str = "EFBIG";

/// No space left on device.
pub const ENOSPC: std::os::raw::c_int = 28;
/// Human-readable message for ENOSPC.
pub const ENOSPC_MSG: &str = "No space left on device";
/// Identifier for ENOSPC as a string (equals to "ENOSPC").
pub const ENOSPC_ID: &str = "ENOSPC";

/// Illegal seek.
pub const ESPIPE: std::os::raw::c_int = 29;
/// Human-readable message for ESPIPE.
pub const ESPIPE_MSG: &str = "Illegal seek";
/// Identifier for ESPIPE as a string (equals to "ESPIPE").
pub const ESPIPE_ID: &str = "ESPIPE";

/// Read-only file system.
pub const EROFS: std::os::raw::c_int = 30;
/// Human-readable message for EROFS.
pub const EROFS_MSG: &str = "Read-only file system";
/// Identifier for EROFS as a string (equals to "EROFS").
pub const EROFS_ID: &str = "EROFS";

/// Too many links.
pub const EMLINK: std::os::raw::c_int = 31;
/// Human-readable message for EMLINK.
pub const EMLINK_MSG: &str = "Too many links";
/// Identifier for EMLINK as a string (equals to "EMLINK").
pub const EMLINK_ID: &str = "EMLINK";

/// Broken pipe.
pub const EPIPE: std::os::raw::c_int = 32;
/// Human-readable message for EPIPE.
pub const EPIPE_MSG: &str = "Broken pipe";
/// Identifier for EPIPE as a string (equals to "EPIPE").
pub const EPIPE_ID: &str = "EPIPE";

/// Math argument out of domain of func.
pub const EDOM: std::os::raw::c_int = 33;
/// Human-readable message for EDOM.
pub const EDOM_MSG: &str = "Math argument out of domain of func";
/// Identifier for EDOM as a string (equals to "EDOM").
pub const EDOM_ID: &str = "EDOM";

/// Math result not representable.
pub const ERANGE: std::os::raw::c_int = 34;
/// Human-readable message for ERANGE.
pub const ERANGE_MSG: &str = "Math result not representable";
/// Identifier for ERANGE as a string (equals to "ERANGE").
pub const ERANGE_ID: &str = "ERANGE";

/// Resource deadlock would occur.
pub const EDEADLK: std::os::raw::c_int = 35;
/// Human-readable message for EDEADLK.
pub const EDEADLK_MSG: &str = "Resource deadlock would occur";
/// Identifier for EDEADLK as a string (equals to "EDEADLK").
pub const EDEADLK_ID: &str = "EDEADLK";

/// File name too long.
pub const ENAMETOOLONG: std::os::raw::c_int = 36;
/// Human-readable message for ENAMETOOLONG.
pub const ENAMETOOLONG_MSG: &str = "File name too long";
/// Identifier for ENAMETOOLONG as a string (equals to "ENAMETOOLONG").
pub const ENAMETOOLONG_ID: &str = "ENAMETOOLONG";

/// No record locks available.
pub const ENOLCK: std::os::raw::c_int = 37;
/// Human-readable message for ENOLCK.
pub const ENOLCK_MSG: &str = "No record locks available";
/// Identifier for ENOLCK as a string (equals to "ENOLCK").
pub const ENOLCK_ID: &str = "ENOLCK";

/// Invalid system call number.
pub const ENOSYS: std::os::raw::c_int = 38;
/// Human-readable message for ENOSYS.
pub const ENOSYS_MSG: &str = "Invalid system call number";
/// Identifier for ENOSYS as a string (equals to "ENOSYS").
pub const ENOSYS_ID: &str = "ENOSYS";

/// Directory not empty.
pub const ENOTEMPTY: std::os::raw::c_int = 39;
/// Human-readable message for ENOTEMPTY.
pub const ENOTEMPTY_MSG: &str = "Directory not empty";
/// Identifier for ENOTEMPTY as a string (equals to "ENOTEMPTY").
pub const ENOTEMPTY_ID: &str = "ENOTEMPTY";

/// Too many symbolic links encountered.
pub const ELOOP: std::os::raw::c_int = 40;
/// Human-readable message for ELOOP.
pub const ELOOP_MSG: &str = "Too many symbolic links encountered";
/// Identifier for ELOOP as a string (equals to "ELOOP").
pub const ELOOP_ID: &str = "ELOOP";

/// Operation would block.
pub const EWOULDBLOCK: std::os::raw::c_int = EAGAIN;
/// Human-readable message for EWOULDBLOCK.
pub const EWOULDBLOCK_MSG: &str = "Operation would block";
/// Identifier for EWOULDBLOCK as a string (equals to "EWOULDBLOCK").
pub const EWOULDBLOCK_ID: &str = "EWOULDBLOCK";

/// No message of desired type.
pub const ENOMSG: std::os::raw::c_int = 42;
/// Human-readable message for ENOMSG.
pub const ENOMSG_MSG: &str = "No message of desired type";
/// Identifier for ENOMSG as a string (equals to "ENOMSG").
pub const ENOMSG_ID: &str = "ENOMSG";

/// Identifier removed.
pub const EIDRM: std::os::raw::c_int = 43;
/// Human-readable message for EIDRM.
pub const EIDRM_MSG: &str = "Identifier removed";
/// Identifier for EIDRM as a string (equals to "EIDRM").
pub const EIDRM_ID: &str = "EIDRM";

/// Channel number out of range.
pub const ECHRNG: std::os::raw::c_int = 44;
/// Human-readable message for ECHRNG.
pub const ECHRNG_MSG: &str = "Channel number out of range";
/// Identifier for ECHRNG as a string (equals to "ECHRNG").
pub const ECHRNG_ID: &str = "ECHRNG";

/// Level 2 not synchronized.
pub const EL2NSYNC: std::os::raw::c_int = 45;
/// Human-readable message for EL2NSYNC.
pub const EL2NSYNC_MSG: &str = "Level 2 not synchronized";
/// Identifier for EL2NSYNC as a string (equals to "EL2NSYNC").
pub const EL2NSYNC_ID: &str = "EL2NSYNC";

/// Level 3 halted.
pub const EL3HLT: std::os::raw::c_int = 46;
/// Human-readable message for EL3HLT.
pub const EL3HLT_MSG: &str = "Level 3 halted";
/// Identifier for EL3HLT as a string (equals to "EL3HLT").
pub const EL3HLT_ID: &str = "EL3HLT";

/// Level 3 reset.
pub const EL3RST: std::os::raw::c_int = 47;
/// Human-readable message for EL3RST.
pub const EL3RST_MSG: &str = "Level 3 reset";
/// Identifier for EL3RST as a string (equals to "EL3RST").
pub const EL3RST_ID: &str = "EL3RST";

/// Link number out of range.
pub const ELNRNG: std::os::raw::c_int = 48;
/// Human-readable message for ELNRNG.
pub const ELNRNG_MSG: &str = "Link number out of range";
/// Identifier for ELNRNG as a string (equals to "ELNRNG").
pub const ELNRNG_ID: &str = "ELNRNG";

/// Protocol driver not attached.
pub const EUNATCH: std::os::raw::c_int = 49;
/// Human-readable message for EUNATCH.
pub const EUNATCH_MSG: &str = "Protocol driver not attached";
/// Identifier for EUNATCH as a string (equals to "EUNATCH").
pub const EUNATCH_ID: &str = "EUNATCH";

/// No CSI structure available.
pub const ENOCSI: std::os::raw::c_int = 50;
/// Human-readable message for ENOCSI.
pub const ENOCSI_MSG: &str = "No CSI structure available";
/// Identifier for ENOCSI as a string (equals to "ENOCSI").
pub const ENOCSI_ID: &str = "ENOCSI";

/// Level 2 halted.
pub const EL2HLT: std::os::raw::c_int = 51;
/// Human-readable message for EL2HLT.
pub const EL2HLT_MSG: &str = "Level 2 halted";
/// Identifier for EL2HLT as a string (equals to "EL2HLT").
pub const EL2HLT_ID: &str = "EL2HLT";

/// Invalid exchange.
pub const EBADE: std::os::raw::c_int = 52;
/// Human-readable message for EBADE.
pub const EBADE_MSG: &str = "Invalid exchange";
/// Identifier for EBADE as a string (equals to "EBADE").
pub const EBADE_ID: &str = "EBADE";

/// Invalid request descriptor.
pub const EBADR: std::os::raw::c_int = 53;
/// Human-readable message for EBADR.
pub const EBADR_MSG: &str = "Invalid request descriptor";
/// Identifier for EBADR as a string (equals to "EBADR").
pub const EBADR_ID: &str = "EBADR";

/// Exchange full.
pub const EXFULL: std::os::raw::c_int = 54;
/// Human-readable message for EXFULL.
pub const EXFULL_MSG: &str = "Exchange full";
/// Identifier for EXFULL as a string (equals to "EXFULL").
pub const EXFULL_ID: &str = "EXFULL";

/// No anode.
pub const ENOANO: std::os::raw::c_int = 55;
/// Human-readable message for ENOANO.
pub const ENOANO_MSG: &str = "No anode";
/// Identifier for ENOANO as a string (equals to "ENOANO").
pub const ENOANO_ID: &str = "ENOANO";

/// Invalid request code.
pub const EBADRQC: std::os::raw::c_int = 56;
/// Human-readable message for EBADRQC.
pub const EBADRQC_MSG: &str = "Invalid request code";
/// Identifier for EBADRQC as a string (equals to "EBADRQC").
pub const EBADRQC_ID: &str = "EBADRQC";

/// Invalid slot.
pub const EBADSLT: std::os::raw::c_int = 57;
/// Human-readable message for EBADSLT.
pub const EBADSLT_MSG: &str = "Invalid slot";
/// Identifier for EBADSLT as a string (equals to "EBADSLT").
pub const EBADSLT_ID: &str = "EBADSLT";

/// Same as EDEADLK (Resource deadlock would occur).
pub const EDEADLOCK: std::os::raw::c_int = EDEADLK;
/// Human-readable message for EDEADLOCK.
pub const EDEADLOCK_MSG: &str = "Same as EDEADLK (Resource deadlock would occur)";
/// Identifier for EDEADLOCK as a string (equals to "EDEADLOCK").
pub const EDEADLOCK_ID: &str = "EDEADLOCK";

/// Bad font file format.
pub const EBFONT: std::os::raw::c_int = 59;
/// Human-readable message for EBFONT.
pub const EBFONT_MSG: &str = "Bad font file format";
/// Identifier for EBFONT as a string (equals to "EBFONT").
pub const EBFONT_ID: &str = "EBFONT";

/// Device not a stream.
pub const ENOSTR: std::os::raw::c_int = 60;
/// Human-readable message for ENOSTR.
pub const ENOSTR_MSG: &str = "Device not a stream";
/// Identifier for ENOSTR as a string (equals to "ENOSTR").
pub const ENOSTR_ID: &str = "ENOSTR";

/// No data available.
pub const ENODATA: std::os::raw::c_int = 61;
/// Human-readable message for ENODATA.
pub const ENODATA_MSG: &str = "No data available";
/// Identifier for ENODATA as a string (equals to "ENODATA").
pub const ENODATA_ID: &str = "ENODATA";

/// Timer expired.
pub const ETIME: std::os::raw::c_int = 62;
/// Human-readable message for ETIME.
pub const ETIME_MSG: &str = "Timer expired";
/// Identifier for ETIME as a string (equals to "ETIME").
pub const ETIME_ID: &str = "ETIME";

/// Out of streams resources.
pub const ENOSR: std::os::raw::c_int = 63;
/// Human-readable message for ENOSR.
pub const ENOSR_MSG: &str = "Out of streams resources";
/// Identifier for ENOSR as a string (equals to "ENOSR").
pub const ENOSR_ID: &str = "ENOSR";

/// Machine is not on the network.
pub const ENONET: std::os::raw::c_int = 64;
/// Human-readable message for ENONET.
pub const ENONET_MSG: &str = "Machine is not on the network";
/// Identifier for ENONET as a string (equals to "ENONET").
pub const ENONET_ID: &str = "ENONET";

/// Package not installed.
pub const ENOPKG: std::os::raw::c_int = 65;
/// Human-readable message for ENOPKG.
pub const ENOPKG_MSG: &str = "Package not installed";
/// Identifier for ENOPKG as a string (equals to "ENOPKG").
pub const ENOPKG_ID: &str = "ENOPKG";

/// Object is remote.
pub const EREMOTE: std::os::raw::c_int = 66;
/// Human-readable message for EREMOTE.
pub const EREMOTE_MSG: &str = "Object is remote";
/// Identifier for EREMOTE as a string (equals to "EREMOTE").
pub const EREMOTE_ID: &str = "EREMOTE";

/// Link has been severed.
pub const ENOLINK: std::os::raw::c_int = 67;
/// Human-readable message for ENOLINK.
pub const ENOLINK_MSG: &str = "Link has been severed";
/// Identifier for ENOLINK as a string (equals to "ENOLINK").
pub const ENOLINK_ID: &str = "ENOLINK";

/// Advertise error.
pub const EADV: std::os::raw::c_int = 68;
/// Human-readable message for EADV.
pub const EADV_MSG: &str = "Advertise error";
/// Identifier for EADV as a string (equals to "EADV").
pub const EADV_ID: &str = "EADV";

/// Srmount error.
pub const ESRMNT: std::os::raw::c_int = 69;
/// Human-readable message for ESRMNT.
pub const ESRMNT_MSG: &str = "Srmount error";
/// Identifier for ESRMNT as a string (equals to "ESRMNT").
pub const ESRMNT_ID: &str = "ESRMNT";

/// Communication error on send.
pub const ECOMM: std::os::raw::c_int = 70;
/// Human-readable message for ECOMM.
pub const ECOMM_MSG: &str = "Communication error on send";
/// Identifier for ECOMM as a string (equals to "ECOMM").
pub const ECOMM_ID: &str = "ECOMM";

/// Protocol error.
pub const EPROTO: std::os::raw::c_int = 71;
/// Human-readable message for EPROTO.
pub const EPROTO_MSG: &str = "Protocol error";
/// Identifier for EPROTO as a string (equals to "EPROTO").
pub const EPROTO_ID: &str = "EPROTO";

/// Multihop attempted.
pub const EMULTIHOP: std::os::raw::c_int = 72;
/// Human-readable message for EMULTIHOP.
pub const EMULTIHOP_MSG: &str = "Multihop attempted";
/// Identifier for EMULTIHOP as a string (equals to "EMULTIHOP").
pub const EMULTIHOP_ID: &str = "EMULTIHOP";

/// RFS specific error.
pub const EDOTDOT: std::os::raw::c_int = 73;
/// Human-readable message for EDOTDOT.
pub const EDOTDOT_MSG: &str = "RFS specific error";
/// Identifier for EDOTDOT as a string (equals to "EDOTDOT").
pub const EDOTDOT_ID: &str = "EDOTDOT";

/// Not a data message.
pub const EBADMSG: std::os::raw::c_int = 74;
/// Human-readable message for EBADMSG.
pub const EBADMSG_MSG: &str = "Not a data message";
/// Identifier for EBADMSG as a string (equals to "EBADMSG").
pub const EBADMSG_ID: &str = "EBADMSG";

/// Value too large for defined data type.
pub const EOVERFLOW: std::os::raw::c_int = 75;
/// Human-readable message for EOVERFLOW.
pub const EOVERFLOW_MSG: &str = "Value too large for defined data type";
/// Identifier for EOVERFLOW as a string (equals to "EOVERFLOW").
pub const EOVERFLOW_ID: &str = "EOVERFLOW";

/// Name not unique on network.
pub const ENOTUNIQ: std::os::raw::c_int = 76;
/// Human-readable message for ENOTUNIQ.
pub const ENOTUNIQ_MSG: &str = "Name not unique on network";
/// Identifier for ENOTUNIQ as a string (equals to "ENOTUNIQ").
pub const ENOTUNIQ_ID: &str = "ENOTUNIQ";

/// File descriptor in bad state.
pub const EBADFD: std::os::raw::c_int = 77;
/// Human-readable message for EBADFD.
pub const EBADFD_MSG: &str = "File descriptor in bad state";
/// Identifier for EBADFD as a string (equals to "EBADFD").
pub const EBADFD_ID: &str = "EBADFD";

/// Remote address changed.
pub const EREMCHG: std::os::raw::c_int = 78;
/// Human-readable message for EREMCHG.
pub const EREMCHG_MSG: &str = "Remote address changed";
/// Identifier for EREMCHG as a string (equals to "EREMCHG").
pub const EREMCHG_ID: &str = "EREMCHG";

/// Can not access a needed shared library.
pub const ELIBACC: std::os::raw::c_int = 79;
/// Human-readable message for ELIBACC.
pub const ELIBACC_MSG: &str = "Can not access a needed shared library";
/// Identifier for ELIBACC as a string (equals to "ELIBACC").
pub const ELIBACC_ID: &str = "ELIBACC";

/// Accessing a corrupted shared library.
pub const ELIBBAD: std::os::raw::c_int = 80;
/// Human-readable message for ELIBBAD.
pub const ELIBBAD_MSG: &str = "Accessing a corrupted shared library";
/// Identifier for ELIBBAD as a string (equals to "ELIBBAD").
pub const ELIBBAD_ID: &str = "ELIBBAD";

/// .lib section in a.out corrupted.
pub const ELIBSCN: std::os::raw::c_int = 81;
/// Human-readable message for ELIBSCN.
pub const ELIBSCN_MSG: &str = ".lib section in a.out corrupted";
/// Identifier for ELIBSCN as a string (equals to "ELIBSCN").
pub const ELIBSCN_ID: &str = "ELIBSCN";

/// Attempting to link in too many shared libraries.
pub const ELIBMAX: std::os::raw::c_int = 82;
/// Human-readable message for ELIBMAX.
pub const ELIBMAX_MSG: &str = "Attempting to link in too many shared libraries";
/// Identifier for ELIBMAX as a string (equals to "ELIBMAX").
pub const ELIBMAX_ID: &str = "ELIBMAX";

/// Cannot exec a shared library directly.
pub const ELIBEXEC: std::os::raw::c_int = 83;
/// Human-readable message for ELIBEXEC.
pub const ELIBEXEC_MSG: &str = "Cannot exec a shared library directly";
/// Identifier for ELIBEXEC as a string (equals to "ELIBEXEC").
pub const ELIBEXEC_ID: &str = "ELIBEXEC";

/// Illegal byte sequence.
pub const EILSEQ: std::os::raw::c_int = 84;
/// Human-readable message for EILSEQ.
pub const EILSEQ_MSG: &str = "Illegal byte sequence";
/// Identifier for EILSEQ as a string (equals to "EILSEQ").
pub const EILSEQ_ID: &str = "EILSEQ";

/// Interrupted system call should be restarted.
pub const ERESTART: std::os::raw::c_int = 85;
/// Human-readable message for ERESTART.
pub const ERESTART_MSG: &str = "Interrupted system call should be restarted";
/// Identifier for ERESTART as a string (equals to "ERESTART").
pub const ERESTART_ID: &str = "ERESTART";

/// Streams pipe error.
pub const ESTRPIPE: std::os::raw::c_int = 86;
/// Human-readable message for ESTRPIPE.
pub const ESTRPIPE_MSG: &str = "Streams pipe error";
/// Identifier for ESTRPIPE as a string (equals to "ESTRPIPE").
pub const ESTRPIPE_ID: &str = "ESTRPIPE";

/// Too many users.
pub const EUSERS: std::os::raw::c_int = 87;
/// Human-readable message for EUSERS.
pub const EUSERS_MSG: &str = "Too many users";
/// Identifier for EUSERS as a string (equals to "EUSERS").
pub const EUSERS_ID: &str = "EUSERS";

/// Socket operation on non-socket.
pub const ENOTSOCK: std::os::raw::c_int = 88;
/// Human-readable message for ENOTSOCK.
pub const ENOTSOCK_MSG: &str = "Socket operation on non-socket";
/// Identifier for ENOTSOCK as a string (equals to "ENOTSOCK").
pub const ENOTSOCK_ID: &str = "ENOTSOCK";

/// Destination address required.
pub const EDESTADDRREQ: std::os::raw::c_int = 89;
/// Human-readable message for EDESTADDRREQ.
pub const EDESTADDRREQ_MSG: &str = "Destination address required";
/// Identifier for EDESTADDRREQ as a string (equals to "EDESTADDRREQ").
pub const EDESTADDRREQ_ID: &str = "EDESTADDRREQ";

/// Message too long.
pub const EMSGSIZE: std::os::raw::c_int = 90;
/// Human-readable message for EMSGSIZE.
pub const EMSGSIZE_MSG: &str = "Message too long";
/// Identifier for EMSGSIZE as a string (equals to "EMSGSIZE").
pub const EMSGSIZE_ID: &str = "EMSGSIZE";

/// Protocol wrong type for socket.
pub const EPROTOTYPE: std::os::raw::c_int = 91;
/// Human-readable message for EPROTOTYPE.
pub const EPROTOTYPE_MSG: &str = "Protocol wrong type for socket";
/// Identifier for EPROTOTYPE as a string (equals to "EPROTOTYPE").
pub const EPROTOTYPE_ID: &str = "EPROTOTYPE";

/// Protocol not available.
pub const ENOPROTOOPT: std::os::raw::c_int = 92;
/// Human-readable message for ENOPROTOOPT.
pub const ENOPROTOOPT_MSG: &str = "Protocol not available";
/// Identifier for ENOPROTOOPT as a string (equals to "ENOPROTOOPT").
pub const ENOPROTOOPT_ID: &str = "ENOPROTOOPT";

/// Protocol not supported.
pub const EPROTONOSUPPORT: std::os::raw::c_int = 93;
/// Human-readable message for EPROTONOSUPPORT.
pub const EPROTONOSUPPORT_MSG: &str = "Protocol not supported";
/// Identifier for EPROTONOSUPPORT as a string (equals to "EPROTONOSUPPORT").
pub const EPROTONOSUPPORT_ID: &str = "EPROTONOSUPPORT";

/// Socket type not supported.
pub const ESOCKTNOSUPPORT: std::os::raw::c_int = 94;
/// Human-readable message for ESOCKTNOSUPPORT.
pub const ESOCKTNOSUPPORT_MSG: &str = "Socket type not supported";
/// Identifier for ESOCKTNOSUPPORT as a string (equals to "ESOCKTNOSUPPORT").
pub const ESOCKTNOSUPPORT_ID: &str = "ESOCKTNOSUPPORT";

/// Operation not supported on transport endpoint.
pub const EOPNOTSUPP: std::os::raw::c_int = 95;
/// Human-readable message for EOPNOTSUPP.
pub const EOPNOTSUPP_MSG: &str = "Operation not supported on transport endpoint";
/// Identifier for EOPNOTSUPP as a string (equals to "EOPNOTSUPP").
pub const EOPNOTSUPP_ID: &str = "EOPNOTSUPP";

/// Protocol family not supported.
pub const EPFNOSUPPORT: std::os::raw::c_int = 96;
/// Human-readable message for EPFNOSUPPORT.
pub const EPFNOSUPPORT_MSG: &str = "Protocol family not supported";
/// Identifier for EPFNOSUPPORT as a string (equals to "EPFNOSUPPORT").
pub const EPFNOSUPPORT_ID: &str = "EPFNOSUPPORT";

/// Address family not supported by protocol.
pub const EAFNOSUPPORT: std::os::raw::c_int = 97;
/// Human-readable message for EAFNOSUPPORT.
pub const EAFNOSUPPORT_MSG: &str = "Address family not supported by protocol";
/// Identifier for EAFNOSUPPORT as a string (equals to "EAFNOSUPPORT").
pub const EAFNOSUPPORT_ID: &str = "EAFNOSUPPORT";

/// Address already in use.
pub const EADDRINUSE: std::os::raw::c_int = 98;
/// Human-readable message for EADDRINUSE.
pub const EADDRINUSE_MSG: &str = "Address already in use";
/// Identifier for EADDRINUSE as a string (equals to "EADDRINUSE").
pub const EADDRINUSE_ID: &str = "EADDRINUSE";

/// Cannot assign requested address.
pub const EADDRNOTAVAIL: std::os::raw::c_int = 99;
/// Human-readable message for EADDRNOTAVAIL.
pub const EADDRNOTAVAIL_MSG: &str = "Cannot assign requested address";
/// Identifier for EADDRNOTAVAIL as a string (equals to "EADDRNOTAVAIL").
pub const EADDRNOTAVAIL_ID: &str = "EADDRNOTAVAIL";

/// Network is down.
pub const ENETDOWN: std::os::raw::c_int = 100;
/// Human-readable message for ENETDOWN.
pub const ENETDOWN_MSG: &str = "Network is down";
/// Identifier for ENETDOWN as a string (equals to "ENETDOWN").
pub const ENETDOWN_ID: &str = "ENETDOWN";

/// Network is unreachable.
pub const ENETUNREACH: std::os::raw::c_int = 101;
/// Human-readable message for ENETUNREACH.
pub const ENETUNREACH_MSG: &str = "Network is unreachable";
/// Identifier for ENETUNREACH as a string (equals to "ENETUNREACH").
pub const ENETUNREACH_ID: &str = "ENETUNREACH";

/// Network dropped connection because of reset.
pub const ENETRESET: std::os::raw::c_int = 102;
/// Human-readable message for ENETRESET.
pub const ENETRESET_MSG: &str = "Network dropped connection because of reset";
/// Identifier for ENETRESET as a string (equals to "ENETRESET").
pub const ENETRESET_ID: &str = "ENETRESET";

/// Software caused connection abort.
pub const ECONNABORTED: std::os::raw::c_int = 103;
/// Human-readable message for ECONNABORTED.
pub const ECONNABORTED_MSG: &str = "Software caused connection abort";
/// Identifier for ECONNABORTED as a string (equals to "ECONNABORTED").
pub const ECONNABORTED_ID: &str = "ECONNABORTED";

/// Connection reset by peer.
pub const ECONNRESET: std::os::raw::c_int = 104;
/// Human-readable message for ECONNRESET.
pub const ECONNRESET_MSG: &str = "Connection reset by peer";
/// Identifier for ECONNRESET as a string (equals to "ECONNRESET").
pub const ECONNRESET_ID: &str = "ECONNRESET";

/// No buffer space available.
pub const ENOBUFS: std::os::raw::c_int = 105;
/// Human-readable message for ENOBUFS.
pub const ENOBUFS_MSG: &str = "No buffer space available";
/// Identifier for ENOBUFS as a string (equals to "ENOBUFS").
pub const ENOBUFS_ID: &str = "ENOBUFS";

/// Transport endpoint is already connected.
pub const EISCONN: std::os::raw::c_int = 106;
/// Human-readable message for EISCONN.
pub const EISCONN_MSG: &str = "Transport endpoint is already connected";
/// Identifier for EISCONN as a string (equals to "EISCONN").
pub const EISCONN_ID: &str = "EISCONN";

/// Transport endpoint is not connected.
pub const ENOTCONN: std::os::raw::c_int = 107;
/// Human-readable message for ENOTCONN.
pub const ENOTCONN_MSG: &str = "Transport endpoint is not connected";
/// Identifier for ENOTCONN as a string (equals to "ENOTCONN").
pub const ENOTCONN_ID: &str = "ENOTCONN";

/// Cannot send after transport endpoint shutdown.
pub const ESHUTDOWN: std::os::raw::c_int = 108;
/// Human-readable message for ESHUTDOWN.
pub const ESHUTDOWN_MSG: &str = "Cannot send after transport endpoint shutdown";
/// Identifier for ESHUTDOWN as a string (equals to "ESHUTDOWN").
pub const ESHUTDOWN_ID: &str = "ESHUTDOWN";

/// Too many references: cannot splice.
pub const ETOOMANYREFS: std::os::raw::c_int = 109;
/// Human-readable message for ETOOMANYREFS.
pub const ETOOMANYREFS_MSG: &str = "Too many references: cannot splice";
/// Identifier for ETOOMANYREFS as a string (equals to "ETOOMANYREFS").
pub const ETOOMANYREFS_ID: &str = "ETOOMANYREFS";

/// Connection timed out.
pub const ETIMEDOUT: std::os::raw::c_int = 110;
/// Human-readable message for ETIMEDOUT.
pub const ETIMEDOUT_MSG: &str = "Connection timed out";
/// Identifier for ETIMEDOUT as a string (equals to "ETIMEDOUT").
pub const ETIMEDOUT_ID: &str = "ETIMEDOUT";

/// Connection refused.
pub const ECONNREFUSED: std::os::raw::c_int = 111;
/// Human-readable message for ECONNREFUSED.
pub const ECONNREFUSED_MSG: &str = "Connection refused";
/// Identifier for ECONNREFUSED as a string (equals to "ECONNREFUSED").
pub const ECONNREFUSED_ID: &str = "ECONNREFUSED";

/// Host is down.
pub const EHOSTDOWN: std::os::raw::c_int = 112;
/// Human-readable message for EHOSTDOWN.
pub const EHOSTDOWN_MSG: &str = "Host is down";
/// Identifier for EHOSTDOWN as a string (equals to "EHOSTDOWN").
pub const EHOSTDOWN_ID: &str = "EHOSTDOWN";

/// No route to host.
pub const EHOSTUNREACH: std::os::raw::c_int = 113;
/// Human-readable message for EHOSTUNREACH.
pub const EHOSTUNREACH_MSG: &str = "No route to host";
/// Identifier for EHOSTUNREACH as a string (equals to "EHOSTUNREACH").
pub const EHOSTUNREACH_ID: &str = "EHOSTUNREACH";

/// Operation already in progress.
pub const EALREADY: std::os::raw::c_int = 114;
/// Human-readable message for EALREADY.
pub const EALREADY_MSG: &str = "Operation already in progress";
/// Identifier for EALREADY as a string (equals to "EALREADY").
pub const EALREADY_ID: &str = "EALREADY";

/// Operation now in progress.
pub const EINPROGRESS: std::os::raw::c_int = 115;
/// Human-readable message for EINPROGRESS.
pub const EINPROGRESS_MSG: &str = "Operation now in progress";
/// Identifier for EINPROGRESS as a string (equals to "EINPROGRESS").
pub const EINPROGRESS_ID: &str = "EINPROGRESS";

/// Stale file handle.
pub const ESTALE: std::os::raw::c_int = 116;
/// Human-readable message for ESTALE.
pub const ESTALE_MSG: &str = "Stale file handle";
/// Identifier for ESTALE as a string (equals to "ESTALE").
pub const ESTALE_ID: &str = "ESTALE";

/// Structure needs cleaning.
pub const EUCLEAN: std::os::raw::c_int = 117;
/// Human-readable message for EUCLEAN.
pub const EUCLEAN_MSG: &str = "Structure needs cleaning";
/// Identifier for EUCLEAN as a string (equals to "EUCLEAN").
pub const EUCLEAN_ID: &str = "EUCLEAN";

/// Not a XENIX named type file.
pub const ENOTNAM: std::os::raw::c_int = 118;
/// Human-readable message for ENOTNAM.
pub const ENOTNAM_MSG: &str = "Not a XENIX named type file";
/// Identifier for ENOTNAM as a string (equals to "ENOTNAM").
pub const ENOTNAM_ID: &str = "ENOTNAM";

/// No XENIX semaphores available.
pub const ENAVAIL: std::os::raw::c_int = 119;
/// Human-readable message for ENAVAIL.
pub const ENAVAIL_MSG: &str = "No XENIX semaphores available";
/// Identifier for ENAVAIL as a string (equals to "ENAVAIL").
pub const ENAVAIL_ID: &str = "ENAVAIL";

/// Is a named type file.
pub const EISNAM: std::os::raw::c_int = 120;
/// Human-readable message for EISNAM.
pub const EISNAM_MSG: &str = "Is a named type file";
/// Identifier for EISNAM as a string (equals to "EISNAM").
pub const EISNAM_ID: &str = "EISNAM";

/// Remote I/O error.
pub const EREMOTEIO: std::os::raw::c_int = 121;
/// Human-readable message for EREMOTEIO.
pub const EREMOTEIO_MSG: &str = "Remote I/O error";
/// Identifier for EREMOTEIO as a string (equals to "EREMOTEIO").
pub const EREMOTEIO_ID: &str = "EREMOTEIO";

/// Quota exceeded.
pub const EDQUOT: std::os::raw::c_int = 122;
/// Human-readable message for EDQUOT.
pub const EDQUOT_MSG: &str = "Quota exceeded";
/// Identifier for EDQUOT as a string (equals to "EDQUOT").
pub const EDQUOT_ID: &str = "EDQUOT";

/// No medium found.
pub const ENOMEDIUM: std::os::raw::c_int = 123;
/// Human-readable message for ENOMEDIUM.
pub const ENOMEDIUM_MSG: &str = "No medium found";
/// Identifier for ENOMEDIUM as a string (equals to "ENOMEDIUM").
pub const ENOMEDIUM_ID: &str = "ENOMEDIUM";

/// Wrong medium type.
pub const EMEDIUMTYPE: std::os::raw::c_int = 124;
/// Human-readable message for EMEDIUMTYPE.
pub const EMEDIUMTYPE_MSG: &str = "Wrong medium type";
/// Identifier for EMEDIUMTYPE as a string (equals to "EMEDIUMTYPE").
pub const EMEDIUMTYPE_ID: &str = "EMEDIUMTYPE";

/// Operation Canceled.
pub const ECANCELED: std::os::raw::c_int = 125;
/// Human-readable message for ECANCELED.
pub const ECANCELED_MSG: &str = "Operation Canceled";
/// Identifier for ECANCELED as a string (equals to "ECANCELED").
pub const ECANCELED_ID: &str = "ECANCELED";

/// Required key not available.
pub const ENOKEY: std::os::raw::c_int = 126;
/// Human-readable message for ENOKEY.
pub const ENOKEY_MSG: &str = "Required key not available";
/// Identifier for ENOKEY as a string (equals to "ENOKEY").
pub const ENOKEY_ID: &str = "ENOKEY";

/// Key has expired.
pub const EKEYEXPIRED: std::os::raw::c_int = 127;
/// Human-readable message for EKEYEXPIRED.
pub const EKEYEXPIRED_MSG: &str = "Key has expired";
/// Identifier for EKEYEXPIRED as a string (equals to "EKEYEXPIRED").
pub const EKEYEXPIRED_ID: &str = "EKEYEXPIRED";

/// Key has been revoked.
pub const EKEYREVOKED: std::os::raw::c_int = 128;
/// Human-readable message for EKEYREVOKED.
pub const EKEYREVOKED_MSG: &str = "Key has been revoked";
/// Identifier for EKEYREVOKED as a string (equals to "EKEYREVOKED").
pub const EKEYREVOKED_ID: &str = "EKEYREVOKED";

/// Key was rejected by service.
pub const EKEYREJECTED: std::os::raw::c_int = 129;
/// Human-readable message for EKEYREJECTED.
pub const EKEYREJECTED_MSG: &str = "Key was rejected by service";
/// Identifier for EKEYREJECTED as a string (equals to "EKEYREJECTED").
pub const EKEYREJECTED_ID: &str = "EKEYREJECTED";

/// Owner died.
pub const EOWNERDEAD: std::os::raw::c_int = 130;
/// Human-readable message for EOWNERDEAD.
pub const EOWNERDEAD_MSG: &str = "Owner died";
/// Identifier for EOWNERDEAD as a string (equals to "EOWNERDEAD").
pub const EOWNERDEAD_ID: &str = "EOWNERDEAD";

/// State not recoverable.
pub const ENOTRECOVERABLE: std::os::raw::c_int = 131;
/// Human-readable message for ENOTRECOVERABLE.
pub const ENOTRECOVERABLE_MSG: &str = "State not recoverable";
/// Identifier for ENOTRECOVERABLE as a string (equals to "ENOTRECOVERABLE").
pub const ENOTRECOVERABLE_ID: &str = "ENOTRECOVERABLE";

/// Operation not possible due to RF-kill.
pub const ERFKILL: std::os::raw::c_int = 132;
/// Human-readable message for ERFKILL.
pub const ERFKILL_MSG: &str = "Operation not possible due to RF-kill";
/// Identifier for ERFKILL as a string (equals to "ERFKILL").
pub const ERFKILL_ID: &str = "ERFKILL";

/// Memory page has hardware error.
pub const EHWPOISON: std::os::raw::c_int = 133;
/// Human-readable message for EHWPOISON.
pub const EHWPOISON_MSG: &str = "Memory page has hardware error";
/// Identifier for EHWPOISON as a string (equals to "EHWPOISON").
pub const EHWPOISON_ID: &str = "EHWPOISON";

include!(concat!(env!("OUT_DIR"), "/unix.linux.openrisc.rs"));
