/* automatically generated by rust-bindgen */

pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const _BSD_SOURCE: u32 = 1;
pub const _SVID_SOURCE: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_BSD: u32 = 1;
pub const __USE_SVID: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201103;
pub const __STDC_NO_THREADS__: u32 = 1;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 19;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const _BITS_WCHAR_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const MINIMP3_MAX_SAMPLES_PER_FRAME: u32 = 2304;
pub const MP3D_SEEK_TO_BYTE: u32 = 0;
pub const MP3D_SEEK_TO_SAMPLE: u32 = 1;
pub const MINIMP3_PREDECODE_FRAMES: u32 = 2;
pub const MINIMP3_IO_SIZE: u32 = 131072;
pub const MINIMP3_BUF_SIZE: u32 = 16384;
pub const MINIMP3_ENABLE_RING: u32 = 0;
pub const MP3D_E_PARAM: i32 = -1;
pub const MP3D_E_MEMORY: i32 = -2;
pub const MP3D_E_IOERROR: i32 = -3;
pub const MP3D_E_USER: i32 = -4;
pub type wchar_t = ::std::os::raw::c_int;
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_long;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mp3dec_frame_info_t {
    pub frame_bytes: ::std::os::raw::c_int,
    pub frame_offset: ::std::os::raw::c_int,
    pub channels: ::std::os::raw::c_int,
    pub hz: ::std::os::raw::c_int,
    pub layer: ::std::os::raw::c_int,
    pub bitrate_kbps: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_mp3dec_frame_info_t() {
    assert_eq!(
        ::std::mem::size_of::<mp3dec_frame_info_t>(),
        24usize,
        concat!("Size of: ", stringify!(mp3dec_frame_info_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mp3dec_frame_info_t>(),
        4usize,
        concat!("Alignment of ", stringify!(mp3dec_frame_info_t))
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mp3dec_t {
    pub mdct_overlap: [[f32; 288usize]; 2usize],
    pub qmf_state: [f32; 960usize],
    pub reserv: ::std::os::raw::c_int,
    pub free_format_bytes: ::std::os::raw::c_int,
    pub header: [::std::os::raw::c_uchar; 4usize],
    pub reserv_buf: [::std::os::raw::c_uchar; 511usize],
}
#[test]
fn bindgen_test_layout_mp3dec_t() {
    assert_eq!(
        ::std::mem::size_of::<mp3dec_t>(),
        6668usize,
        concat!("Size of: ", stringify!(mp3dec_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mp3dec_t>(),
        4usize,
        concat!("Alignment of ", stringify!(mp3dec_t))
    );
}
extern "C" {
    pub fn mp3dec_init(dec: *mut mp3dec_t);
}
pub type mp3d_sample_t = i16;
extern "C" {
    pub fn mp3dec_decode_frame(
        dec: *mut mp3dec_t,
        mp3: *const u8,
        mp3_bytes: ::std::os::raw::c_int,
        pcm: *mut mp3d_sample_t,
        info: *mut mp3dec_frame_info_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mp3dec_file_info_t {
    pub buffer: *mut mp3d_sample_t,
    pub samples: usize,
    pub channels: ::std::os::raw::c_int,
    pub hz: ::std::os::raw::c_int,
    pub layer: ::std::os::raw::c_int,
    pub avg_bitrate_kbps: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_mp3dec_file_info_t() {
    assert_eq!(
        ::std::mem::size_of::<mp3dec_file_info_t>(),
        32usize,
        concat!("Size of: ", stringify!(mp3dec_file_info_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mp3dec_file_info_t>(),
        8usize,
        concat!("Alignment of ", stringify!(mp3dec_file_info_t))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mp3dec_map_info_t {
    pub buffer: *const u8,
    pub size: usize,
}
#[test]
fn bindgen_test_layout_mp3dec_map_info_t() {
    assert_eq!(
        ::std::mem::size_of::<mp3dec_map_info_t>(),
        16usize,
        concat!("Size of: ", stringify!(mp3dec_map_info_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mp3dec_map_info_t>(),
        8usize,
        concat!("Alignment of ", stringify!(mp3dec_map_info_t))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mp3dec_frame_t {
    pub sample: u64,
    pub offset: u64,
}
#[test]
fn bindgen_test_layout_mp3dec_frame_t() {
    assert_eq!(
        ::std::mem::size_of::<mp3dec_frame_t>(),
        16usize,
        concat!("Size of: ", stringify!(mp3dec_frame_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mp3dec_frame_t>(),
        8usize,
        concat!("Alignment of ", stringify!(mp3dec_frame_t))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mp3dec_index_t {
    pub frames: *mut mp3dec_frame_t,
    pub num_frames: usize,
    pub capacity: usize,
}
#[test]
fn bindgen_test_layout_mp3dec_index_t() {
    assert_eq!(
        ::std::mem::size_of::<mp3dec_index_t>(),
        24usize,
        concat!("Size of: ", stringify!(mp3dec_index_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mp3dec_index_t>(),
        8usize,
        concat!("Alignment of ", stringify!(mp3dec_index_t))
    );
}
pub type MP3D_READ_CB = ::std::option::Option<
    unsafe extern "C" fn(
        buf: *mut ::std::os::raw::c_void,
        size: usize,
        user_data: *mut ::std::os::raw::c_void,
    ) -> usize,
>;
pub type MP3D_SEEK_CB = ::std::option::Option<
    unsafe extern "C" fn(
        position: u64,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mp3dec_io_t {
    pub read: MP3D_READ_CB,
    pub read_data: *mut ::std::os::raw::c_void,
    pub seek: MP3D_SEEK_CB,
    pub seek_data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_mp3dec_io_t() {
    assert_eq!(
        ::std::mem::size_of::<mp3dec_io_t>(),
        32usize,
        concat!("Size of: ", stringify!(mp3dec_io_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mp3dec_io_t>(),
        8usize,
        concat!("Alignment of ", stringify!(mp3dec_io_t))
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mp3dec_ex_t {
    pub mp3d: mp3dec_t,
    pub file: mp3dec_map_info_t,
    pub io: *mut mp3dec_io_t,
    pub index: mp3dec_index_t,
    pub offset: u64,
    pub samples: u64,
    pub detected_samples: u64,
    pub cur_sample: u64,
    pub start_offset: u64,
    pub end_offset: u64,
    pub info: mp3dec_frame_info_t,
    pub buffer: [mp3d_sample_t; 2304usize],
    pub input_consumed: usize,
    pub input_filled: usize,
    pub is_file: ::std::os::raw::c_int,
    pub seek_method: ::std::os::raw::c_int,
    pub vbr_tag_found: ::std::os::raw::c_int,
    pub free_format_bytes: ::std::os::raw::c_int,
    pub buffer_samples: ::std::os::raw::c_int,
    pub buffer_consumed: ::std::os::raw::c_int,
    pub to_skip: ::std::os::raw::c_int,
    pub start_delay: ::std::os::raw::c_int,
    pub last_error: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_mp3dec_ex_t() {
    assert_eq!(
        ::std::mem::size_of::<mp3dec_ex_t>(),
        11456usize,
        concat!("Size of: ", stringify!(mp3dec_ex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mp3dec_ex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(mp3dec_ex_t))
    );
}
pub type MP3D_ITERATE_CB = ::std::option::Option<
    unsafe extern "C" fn(
        user_data: *mut ::std::os::raw::c_void,
        frame: *const u8,
        frame_size: ::std::os::raw::c_int,
        free_format_bytes: ::std::os::raw::c_int,
        buf_size: usize,
        offset: u64,
        info: *mut mp3dec_frame_info_t,
    ) -> ::std::os::raw::c_int,
>;
pub type MP3D_PROGRESS_CB = ::std::option::Option<
    unsafe extern "C" fn(
        user_data: *mut ::std::os::raw::c_void,
        file_size: usize,
        offset: u64,
        info: *mut mp3dec_frame_info_t,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn mp3dec_load_buf(
        dec: *mut mp3dec_t,
        buf: *const u8,
        buf_size: usize,
        info: *mut mp3dec_file_info_t,
        progress_cb: MP3D_PROGRESS_CB,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mp3dec_load_cb(
        dec: *mut mp3dec_t,
        io: *mut mp3dec_io_t,
        buf: *mut u8,
        buf_size: usize,
        info: *mut mp3dec_file_info_t,
        progress_cb: MP3D_PROGRESS_CB,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mp3dec_iterate_buf(
        buf: *const u8,
        buf_size: usize,
        callback: MP3D_ITERATE_CB,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mp3dec_iterate_cb(
        io: *mut mp3dec_io_t,
        buf: *mut u8,
        buf_size: usize,
        callback: MP3D_ITERATE_CB,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mp3dec_ex_open_buf(
        dec: *mut mp3dec_ex_t,
        buf: *const u8,
        buf_size: usize,
        seek_method: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mp3dec_ex_open_cb(
        dec: *mut mp3dec_ex_t,
        io: *mut mp3dec_io_t,
        seek_method: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mp3dec_ex_close(dec: *mut mp3dec_ex_t);
}
extern "C" {
    pub fn mp3dec_ex_seek(dec: *mut mp3dec_ex_t, position: u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mp3dec_ex_read(dec: *mut mp3dec_ex_t, buf: *mut mp3d_sample_t, samples: usize) -> usize;
}
extern "C" {
    pub fn mp3dec_load(
        dec: *mut mp3dec_t,
        file_name: *const ::std::os::raw::c_char,
        info: *mut mp3dec_file_info_t,
        progress_cb: MP3D_PROGRESS_CB,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mp3dec_iterate(
        file_name: *const ::std::os::raw::c_char,
        callback: MP3D_ITERATE_CB,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mp3dec_ex_open(
        dec: *mut mp3dec_ex_t,
        file_name: *const ::std::os::raw::c_char,
        seek_method: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
