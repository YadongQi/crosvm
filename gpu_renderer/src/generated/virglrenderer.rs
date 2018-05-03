/* automatically generated by rust-bindgen */

#[link(name = "virglrenderer")]
extern "C" {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct virgl_box {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    _unused: [u8; 0],
}
pub type virgl_renderer_gl_context = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct virgl_renderer_gl_ctx_param {
    pub version: ::std::os::raw::c_int,
    pub shared: bool,
    pub major_ver: ::std::os::raw::c_int,
    pub minor_ver: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct virgl_renderer_callbacks {
    pub version: ::std::os::raw::c_int,
    pub write_fence: ::std::option::Option<
        unsafe extern "C" fn(cookie: *mut ::std::os::raw::c_void, fence: u32),
    >,
    pub create_gl_context: ::std::option::Option<
        unsafe extern "C" fn(
            cookie: *mut ::std::os::raw::c_void,
            scanout_idx: ::std::os::raw::c_int,
            param: *mut virgl_renderer_gl_ctx_param,
        ) -> virgl_renderer_gl_context,
    >,
    pub destroy_gl_context: ::std::option::Option<
        unsafe extern "C" fn(cookie: *mut ::std::os::raw::c_void, ctx: virgl_renderer_gl_context),
    >,
    pub make_current: ::std::option::Option<
        unsafe extern "C" fn(
            cookie: *mut ::std::os::raw::c_void,
            scanout_idx: ::std::os::raw::c_int,
            ctx: virgl_renderer_gl_context,
        ) -> ::std::os::raw::c_int,
    >,
}
extern "C" {
    pub fn virgl_renderer_init(
        cookie: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_int,
        cb: *mut virgl_renderer_callbacks,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn virgl_renderer_poll();
}
extern "C" {
    pub fn virgl_renderer_get_cursor_data(
        resource_id: u32,
        width: *mut u32,
        height: *mut u32,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn virgl_renderer_get_rect(
        resource_id: ::std::os::raw::c_int,
        iov: *mut iovec,
        num_iovs: ::std::os::raw::c_uint,
        offset: u32,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn virgl_renderer_get_fd_for_texture(
        tex_id: u32,
        fd: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct virgl_renderer_resource_create_args {
    pub handle: u32,
    pub target: u32,
    pub format: u32,
    pub bind: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub array_size: u32,
    pub last_level: u32,
    pub nr_samples: u32,
    pub flags: u32,
}
extern "C" {
    pub fn virgl_renderer_resource_create(
        args: *mut virgl_renderer_resource_create_args,
        iov: *mut iovec,
        num_iovs: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn virgl_renderer_resource_unref(res_handle: u32);
}
extern "C" {
    pub fn virgl_renderer_context_create(
        handle: u32,
        nlen: u32,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn virgl_renderer_context_destroy(handle: u32);
}
extern "C" {
    pub fn virgl_renderer_submit_cmd(
        buffer: *mut ::std::os::raw::c_void,
        ctx_id: ::std::os::raw::c_int,
        ndw: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn virgl_renderer_transfer_read_iov(
        handle: u32,
        ctx_id: u32,
        level: u32,
        stride: u32,
        layer_stride: u32,
        box_: *mut virgl_box,
        offset: u64,
        iov: *mut iovec,
        iovec_cnt: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn virgl_renderer_transfer_write_iov(
        handle: u32,
        ctx_id: u32,
        level: ::std::os::raw::c_int,
        stride: u32,
        layer_stride: u32,
        box_: *mut virgl_box,
        offset: u64,
        iovec: *mut iovec,
        iovec_cnt: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn virgl_renderer_get_cap_set(set: u32, max_ver: *mut u32, max_size: *mut u32);
}
extern "C" {
    pub fn virgl_renderer_fill_caps(set: u32, version: u32, caps: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn virgl_renderer_resource_attach_iov(
        res_handle: ::std::os::raw::c_int,
        iov: *mut iovec,
        num_iovs: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn virgl_renderer_resource_detach_iov(
        res_handle: ::std::os::raw::c_int,
        iov: *mut *mut iovec,
        num_iovs: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn virgl_renderer_create_fence(
        client_fence_id: ::std::os::raw::c_int,
        ctx_id: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn virgl_renderer_force_ctx_0();
}
extern "C" {
    pub fn virgl_renderer_ctx_attach_resource(
        ctx_id: ::std::os::raw::c_int,
        res_handle: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn virgl_renderer_ctx_detach_resource(
        ctx_id: ::std::os::raw::c_int,
        res_handle: ::std::os::raw::c_int,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct virgl_renderer_resource_info {
    pub handle: u32,
    pub virgl_format: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub flags: u32,
    pub tex_id: u32,
    pub stride: u32,
    pub drm_fourcc: ::std::os::raw::c_int,
}
extern "C" {
    pub fn virgl_renderer_resource_get_info(
        res_handle: ::std::os::raw::c_int,
        info: *mut virgl_renderer_resource_info,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn virgl_renderer_cleanup(cookie: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn virgl_renderer_reset();
}
extern "C" {
    pub fn virgl_renderer_get_poll_fd() -> ::std::os::raw::c_int;
}
