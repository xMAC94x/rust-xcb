/*
 * This file generated automatically from dri2.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(unused_unsafe)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use base;
use base::*;
use ffi;
use ffi::dri2::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;

pub type xcb_dri2_attachment_t = c_uint;//{
    pub const XCB_DRI2_ATTACHMENT_BUFFER_FRONT_LEFT : xcb_dri2_attachment_t = 1;
    pub const XCB_DRI2_ATTACHMENT_BUFFER_BACK_LEFT : xcb_dri2_attachment_t = 2;
    pub const XCB_DRI2_ATTACHMENT_BUFFER_FRONT_RIGHT : xcb_dri2_attachment_t = 3;
    pub const XCB_DRI2_ATTACHMENT_BUFFER_BACK_RIGHT : xcb_dri2_attachment_t = 4;
    pub const XCB_DRI2_ATTACHMENT_BUFFER_DEPTH : xcb_dri2_attachment_t = 5;
    pub const XCB_DRI2_ATTACHMENT_BUFFER_STENCIL : xcb_dri2_attachment_t = 6;
    pub const XCB_DRI2_ATTACHMENT_BUFFER_ACCUM : xcb_dri2_attachment_t = 7;
    pub const XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_LEFT : xcb_dri2_attachment_t = 8;
    pub const XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_RIGHT : xcb_dri2_attachment_t = 9;
    pub const XCB_DRI2_ATTACHMENT_BUFFER_DEPTH_STENCIL : xcb_dri2_attachment_t = 10;
    pub const XCB_DRI2_ATTACHMENT_BUFFER_HIZ : xcb_dri2_attachment_t = 11;
//}

pub type xcb_dri2_driver_type_t = c_uint;//{
    pub const XCB_DRI2_DRIVER_TYPE_DRI : xcb_dri2_driver_type_t = 1;
    pub const XCB_DRI2_DRIVER_TYPE_VDPAU : xcb_dri2_driver_type_t = 2;
//}

pub type xcb_dri2_event_type_t = c_uint;//{
    pub const XCB_DRI2_EVENT_TYPE_EXCHANGE_COMPLETE : xcb_dri2_event_type_t = 1;
    pub const XCB_DRI2_EVENT_TYPE_BLIT_COMPLETE : xcb_dri2_event_type_t = 2;
    pub const XCB_DRI2_EVENT_TYPE_FLIP_COMPLETE : xcb_dri2_event_type_t = 3;
//}
pub struct Dri2Buffer {pub base : base::Struct<xcb_dri2_dri2_buffer_t> }

pub type Dri2BufferIterator = xcb_dri2_dri2_buffer_iterator_t;

pub type AttachFormatIterator = xcb_dri2_attach_format_iterator_t;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, xcb_dri2_query_version_cookie_t> }

/** Opcode for xcb_dri2_query_version. */
pub const XCB_DRI2_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<xcb_dri2_query_version_reply_t> }
fn mk_reply_xcb_dri2_query_version_reply_t(reply:*mut xcb_dri2_query_version_reply_t) -> QueryVersionReply { QueryVersionReply { base : base::mk_reply(reply) } }
pub struct  ConnectCookie<'s> { pub base : base::Cookie<'s, xcb_dri2_connect_cookie_t> }

/** Opcode for xcb_dri2_connect. */
pub const XCB_DRI2_CONNECT : u8 = 1;
pub struct  AuthenticateCookie<'s> { pub base : base::Cookie<'s, xcb_dri2_authenticate_cookie_t> }

/** Opcode for xcb_dri2_authenticate. */
pub const XCB_DRI2_AUTHENTICATE : u8 = 2;
pub struct AuthenticateReply { base:  base::Reply<xcb_dri2_authenticate_reply_t> }
fn mk_reply_xcb_dri2_authenticate_reply_t(reply:*mut xcb_dri2_authenticate_reply_t) -> AuthenticateReply { AuthenticateReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_dri2_create_drawable. */
pub const XCB_DRI2_CREATE_DRAWABLE : u8 = 3;
/** Opcode for xcb_dri2_destroy_drawable. */
pub const XCB_DRI2_DESTROY_DRAWABLE : u8 = 4;
pub struct  GetBuffersCookie<'s> { pub base : base::Cookie<'s, xcb_dri2_get_buffers_cookie_t> }

/** Opcode for xcb_dri2_get_buffers. */
pub const XCB_DRI2_GET_BUFFERS : u8 = 5;
pub struct GetBuffersReply { base:  base::Reply<xcb_dri2_get_buffers_reply_t> }
fn mk_reply_xcb_dri2_get_buffers_reply_t(reply:*mut xcb_dri2_get_buffers_reply_t) -> GetBuffersReply { GetBuffersReply { base : base::mk_reply(reply) } }
pub struct  CopyRegionCookie<'s> { pub base : base::Cookie<'s, xcb_dri2_copy_region_cookie_t> }

/** Opcode for xcb_dri2_copy_region. */
pub const XCB_DRI2_COPY_REGION : u8 = 6;
pub struct CopyRegionReply { base:  base::Reply<xcb_dri2_copy_region_reply_t> }
fn mk_reply_xcb_dri2_copy_region_reply_t(reply:*mut xcb_dri2_copy_region_reply_t) -> CopyRegionReply { CopyRegionReply { base : base::mk_reply(reply) } }
pub struct  GetBuffersWithFormatCookie<'s> { pub base : base::Cookie<'s, xcb_dri2_get_buffers_with_format_cookie_t> }

/** Opcode for xcb_dri2_get_buffers_with_format. */
pub const XCB_DRI2_GET_BUFFERS_WITH_FORMAT : u8 = 7;
pub struct GetBuffersWithFormatReply { base:  base::Reply<xcb_dri2_get_buffers_with_format_reply_t> }
fn mk_reply_xcb_dri2_get_buffers_with_format_reply_t(reply:*mut xcb_dri2_get_buffers_with_format_reply_t) -> GetBuffersWithFormatReply { GetBuffersWithFormatReply { base : base::mk_reply(reply) } }
pub struct  SwapBuffersCookie<'s> { pub base : base::Cookie<'s, xcb_dri2_swap_buffers_cookie_t> }

/** Opcode for xcb_dri2_swap_buffers. */
pub const XCB_DRI2_SWAP_BUFFERS : u8 = 8;
pub struct SwapBuffersReply { base:  base::Reply<xcb_dri2_swap_buffers_reply_t> }
fn mk_reply_xcb_dri2_swap_buffers_reply_t(reply:*mut xcb_dri2_swap_buffers_reply_t) -> SwapBuffersReply { SwapBuffersReply { base : base::mk_reply(reply) } }
pub struct  GetMscCookie<'s> { pub base : base::Cookie<'s, xcb_dri2_get_msc_cookie_t> }

/** Opcode for xcb_dri2_get_msc. */
pub const XCB_DRI2_GET_MSC : u8 = 9;
pub struct GetMscReply { base:  base::Reply<xcb_dri2_get_msc_reply_t> }
fn mk_reply_xcb_dri2_get_msc_reply_t(reply:*mut xcb_dri2_get_msc_reply_t) -> GetMscReply { GetMscReply { base : base::mk_reply(reply) } }
pub struct  WaitMscCookie<'s> { pub base : base::Cookie<'s, xcb_dri2_wait_msc_cookie_t> }

/** Opcode for xcb_dri2_wait_msc. */
pub const XCB_DRI2_WAIT_MSC : u8 = 10;
pub struct WaitMscReply { base:  base::Reply<xcb_dri2_wait_msc_reply_t> }
fn mk_reply_xcb_dri2_wait_msc_reply_t(reply:*mut xcb_dri2_wait_msc_reply_t) -> WaitMscReply { WaitMscReply { base : base::mk_reply(reply) } }
pub struct  WaitSbcCookie<'s> { pub base : base::Cookie<'s, xcb_dri2_wait_sbc_cookie_t> }

/** Opcode for xcb_dri2_wait_sbc. */
pub const XCB_DRI2_WAIT_SBC : u8 = 11;
pub struct WaitSbcReply { base:  base::Reply<xcb_dri2_wait_sbc_reply_t> }
fn mk_reply_xcb_dri2_wait_sbc_reply_t(reply:*mut xcb_dri2_wait_sbc_reply_t) -> WaitSbcReply { WaitSbcReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_dri2_swap_interval. */
pub const XCB_DRI2_SWAP_INTERVAL : u8 = 12;
pub struct  GetParamCookie<'s> { pub base : base::Cookie<'s, xcb_dri2_get_param_cookie_t> }

/** Opcode for xcb_dri2_get_param. */
pub const XCB_DRI2_GET_PARAM : u8 = 13;
pub struct GetParamReply { base:  base::Reply<xcb_dri2_get_param_reply_t> }
fn mk_reply_xcb_dri2_get_param_reply_t(reply:*mut xcb_dri2_get_param_reply_t) -> GetParamReply { GetParamReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_dri2_buffer_swap_complete. */
pub const XCB_DRI2_BUFFER_SWAP_COMPLETE : u8 = 0;
pub struct BufferSwapCompleteEvent {pub base : base::Event<xcb_dri2_buffer_swap_complete_event_t>}
/** Opcode for xcb_dri2_invalidate_buffers. */
pub const XCB_DRI2_INVALIDATE_BUFFERS : u8 = 1;
pub struct InvalidateBuffersEvent {pub base : base::Event<xcb_dri2_invalidate_buffers_event_t>}

impl Dri2Buffer {
  pub fn attachment(&mut self) -> u32 {
    unsafe { accessor!(attachment -> u32, self.base.strct) }
  }

  pub fn name(&mut self) -> u32 {
    unsafe { accessor!(name -> u32, self.base.strct) }
  }

  pub fn pitch(&mut self) -> u32 {
    unsafe { accessor!(pitch -> u32, self.base.strct) }
  }

  pub fn cpp(&mut self) -> u32 {
    unsafe { accessor!(cpp -> u32, self.base.strct) }
  }

  pub fn flags(&mut self) -> u32 {
    unsafe { accessor!(flags -> u32, self.base.strct) }
  }

}

impl Iterator for Dri2BufferIterator {
    type Item = Dri2Buffer;
    fn next(&mut self) -> Option<Dri2Buffer> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_dri2_dri2_buffer_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_dri2_dri2_buffer_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct AttachFormat {pub base : base::Struct<xcb_dri2_attach_format_t> }


impl AttachFormat {
  pub fn attachment(&mut self) -> u32 {
    unsafe { accessor!(attachment -> u32, self.base.strct) }
  }

  pub fn format(&mut self) -> u32 {
    unsafe { accessor!(format -> u32, self.base.strct) }
  }

}

impl Iterator for AttachFormatIterator {
    type Item = AttachFormat;
    fn next(&mut self) -> Option<AttachFormat> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_dri2_attach_format_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_dri2_attach_format_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn QueryVersion<'r> (c : &'r Connection,
                     major_version : u32,
                     minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_query_version(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              major_version : u32,
                              minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_query_version_unchecked(c.get_raw_conn(),
        major_version as u32, //1
        minor_version as u32); //2
    QueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryVersionReply {
  pub fn major_version(&mut self) -> u32 {
    unsafe { accessor!(major_version -> u32, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u32 {
    unsafe { accessor!(minor_version -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_xcb_dri2_query_version_reply_t, QueryVersionReply, xcb_dri2_query_version_reply);

pub struct ConnectReply { base:  base::Reply<xcb_dri2_connect_reply_t> }
fn mk_reply_xcb_dri2_connect_reply_t(reply:*mut xcb_dri2_connect_reply_t) -> ConnectReply { ConnectReply { base : base::mk_reply(reply) } }
pub fn Connect<'r> (c : &'r Connection,
                window : xproto::Window,
                driver_type : u32) -> ConnectCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_connect(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        driver_type as u32); //2
    ConnectCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ConnectUnchecked<'r> (c : &'r Connection,
                         window : xproto::Window,
                         driver_type : u32) -> ConnectCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_connect_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        driver_type as u32); //2
    ConnectCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ConnectReply {
  pub fn driver_name(&mut self) -> String {
    unsafe { accessor!(str, xcb_dri2_connect_driver_name_length, xcb_dri2_connect_driver_name, (*self.base.reply)) }
  }

  pub fn alignment_pad(&mut self) -> Vec<c_void> {
    unsafe { accessor!(c_void, xcb_dri2_connect_alignment_pad_length, xcb_dri2_connect_alignment_pad, (*self.base.reply)) }
  }

  pub fn device_name(&mut self) -> String {
    unsafe { accessor!(str, xcb_dri2_connect_device_name_length, xcb_dri2_connect_device_name, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ConnectCookie<'s>, mk_reply_xcb_dri2_connect_reply_t, ConnectReply, xcb_dri2_connect_reply);

pub fn Authenticate<'r> (c : &'r Connection,
                     window : xproto::Window,
                     magic : u32) -> AuthenticateCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_authenticate(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        magic as u32); //2
    AuthenticateCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AuthenticateUnchecked<'r> (c : &'r Connection,
                              window : xproto::Window,
                              magic : u32) -> AuthenticateCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_authenticate_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        magic as u32); //2
    AuthenticateCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl AuthenticateReply {
  pub fn authenticated(&mut self) -> u32 {
    unsafe { accessor!(authenticated -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(AuthenticateCookie<'s>, mk_reply_xcb_dri2_authenticate_reply_t, AuthenticateReply, xcb_dri2_authenticate_reply);

pub fn CreateDrawableChecked<'r> (c : &'r Connection,
                              drawable : xproto::Drawable) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_create_drawable_checked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateDrawable<'r> (c : &'r Connection,
                       drawable : xproto::Drawable) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_create_drawable(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DestroyDrawableChecked<'r> (c : &'r Connection,
                               drawable : xproto::Drawable) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_destroy_drawable_checked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyDrawable<'r> (c : &'r Connection,
                        drawable : xproto::Drawable) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_destroy_drawable(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetBuffers<'r> (c : &'r Connection,
                   drawable : xproto::Drawable,
                   count : u32,
                   attachments : &[u32]) -> GetBuffersCookie<'r> {
  unsafe {
    let attachments_len = attachments.len();
    let attachments_ptr = attachments.as_ptr();
    let cookie = xcb_dri2_get_buffers(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        count as u32, //2
        attachments_len as u32, //3
        attachments_ptr as *mut u32); //4
    GetBuffersCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetBuffersUnchecked<'r> (c : &'r Connection,
                            drawable : xproto::Drawable,
                            count : u32,
                            attachments : &[u32]) -> GetBuffersCookie<'r> {
  unsafe {
    let attachments_len = attachments.len();
    let attachments_ptr = attachments.as_ptr();
    let cookie = xcb_dri2_get_buffers_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        count as u32, //2
        attachments_len as u32, //3
        attachments_ptr as *mut u32); //4
    GetBuffersCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetBuffersReply {
  pub fn width(&mut self) -> u32 {
    unsafe { accessor!(width -> u32, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> u32 {
    unsafe { accessor!(height -> u32, (*self.base.reply)) }
  }

  pub fn buffers(&mut self) -> Dri2BufferIterator {
    unsafe { accessor!(Dri2BufferIterator, xcb_dri2_get_buffers_buffers_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetBuffersCookie<'s>, mk_reply_xcb_dri2_get_buffers_reply_t, GetBuffersReply, xcb_dri2_get_buffers_reply);

pub fn CopyRegion<'r> (c : &'r Connection,
                   drawable : xproto::Drawable,
                   region : u32,
                   dest : u32,
                   src : u32) -> CopyRegionCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_copy_region(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        region as u32, //2
        dest as u32, //3
        src as u32); //4
    CopyRegionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CopyRegionUnchecked<'r> (c : &'r Connection,
                            drawable : xproto::Drawable,
                            region : u32,
                            dest : u32,
                            src : u32) -> CopyRegionCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_copy_region_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        region as u32, //2
        dest as u32, //3
        src as u32); //4
    CopyRegionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CopyRegionReply {
}
impl_reply_cookie!(CopyRegionCookie<'s>, mk_reply_xcb_dri2_copy_region_reply_t, CopyRegionReply, xcb_dri2_copy_region_reply);

pub fn GetBuffersWithFormat<'r> (c : &'r Connection,
                             drawable : xproto::Drawable,
                             count : u32,
                             attachments : &[AttachFormat]) -> GetBuffersWithFormatCookie<'r> {
  unsafe {
    let attachments_len = attachments.len();
    let attachments_ptr = attachments.as_ptr();
    let cookie = xcb_dri2_get_buffers_with_format(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        count as u32, //2
        attachments_len as u32, //3
        attachments_ptr as *mut xcb_dri2_attach_format_t); //4
    GetBuffersWithFormatCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetBuffersWithFormatUnchecked<'r> (c : &'r Connection,
                                      drawable : xproto::Drawable,
                                      count : u32,
                                      attachments : &[AttachFormat]) -> GetBuffersWithFormatCookie<'r> {
  unsafe {
    let attachments_len = attachments.len();
    let attachments_ptr = attachments.as_ptr();
    let cookie = xcb_dri2_get_buffers_with_format_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        count as u32, //2
        attachments_len as u32, //3
        attachments_ptr as *mut xcb_dri2_attach_format_t); //4
    GetBuffersWithFormatCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetBuffersWithFormatReply {
  pub fn width(&mut self) -> u32 {
    unsafe { accessor!(width -> u32, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> u32 {
    unsafe { accessor!(height -> u32, (*self.base.reply)) }
  }

  pub fn buffers(&mut self) -> Dri2BufferIterator {
    unsafe { accessor!(Dri2BufferIterator, xcb_dri2_get_buffers_with_format_buffers_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetBuffersWithFormatCookie<'s>, mk_reply_xcb_dri2_get_buffers_with_format_reply_t, GetBuffersWithFormatReply, xcb_dri2_get_buffers_with_format_reply);

pub fn SwapBuffers<'r> (c : &'r Connection,
                    drawable : xproto::Drawable,
                    target_msc_hi : u32,
                    target_msc_lo : u32,
                    divisor_hi : u32,
                    divisor_lo : u32,
                    remainder_hi : u32,
                    remainder_lo : u32) -> SwapBuffersCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_swap_buffers(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        target_msc_hi as u32, //2
        target_msc_lo as u32, //3
        divisor_hi as u32, //4
        divisor_lo as u32, //5
        remainder_hi as u32, //6
        remainder_lo as u32); //7
    SwapBuffersCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SwapBuffersUnchecked<'r> (c : &'r Connection,
                             drawable : xproto::Drawable,
                             target_msc_hi : u32,
                             target_msc_lo : u32,
                             divisor_hi : u32,
                             divisor_lo : u32,
                             remainder_hi : u32,
                             remainder_lo : u32) -> SwapBuffersCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_swap_buffers_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        target_msc_hi as u32, //2
        target_msc_lo as u32, //3
        divisor_hi as u32, //4
        divisor_lo as u32, //5
        remainder_hi as u32, //6
        remainder_lo as u32); //7
    SwapBuffersCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SwapBuffersReply {
  pub fn swap_hi(&mut self) -> u32 {
    unsafe { accessor!(swap_hi -> u32, (*self.base.reply)) }
  }

  pub fn swap_lo(&mut self) -> u32 {
    unsafe { accessor!(swap_lo -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(SwapBuffersCookie<'s>, mk_reply_xcb_dri2_swap_buffers_reply_t, SwapBuffersReply, xcb_dri2_swap_buffers_reply);

pub fn GetMsc<'r> (c : &'r Connection,
               drawable : xproto::Drawable) -> GetMscCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_get_msc(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t); //1
    GetMscCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetMscUnchecked<'r> (c : &'r Connection,
                        drawable : xproto::Drawable) -> GetMscCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_get_msc_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t); //1
    GetMscCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetMscReply {
  pub fn ust_hi(&mut self) -> u32 {
    unsafe { accessor!(ust_hi -> u32, (*self.base.reply)) }
  }

  pub fn ust_lo(&mut self) -> u32 {
    unsafe { accessor!(ust_lo -> u32, (*self.base.reply)) }
  }

  pub fn msc_hi(&mut self) -> u32 {
    unsafe { accessor!(msc_hi -> u32, (*self.base.reply)) }
  }

  pub fn msc_lo(&mut self) -> u32 {
    unsafe { accessor!(msc_lo -> u32, (*self.base.reply)) }
  }

  pub fn sbc_hi(&mut self) -> u32 {
    unsafe { accessor!(sbc_hi -> u32, (*self.base.reply)) }
  }

  pub fn sbc_lo(&mut self) -> u32 {
    unsafe { accessor!(sbc_lo -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetMscCookie<'s>, mk_reply_xcb_dri2_get_msc_reply_t, GetMscReply, xcb_dri2_get_msc_reply);

pub fn WaitMsc<'r> (c : &'r Connection,
                drawable : xproto::Drawable,
                target_msc_hi : u32,
                target_msc_lo : u32,
                divisor_hi : u32,
                divisor_lo : u32,
                remainder_hi : u32,
                remainder_lo : u32) -> WaitMscCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_wait_msc(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        target_msc_hi as u32, //2
        target_msc_lo as u32, //3
        divisor_hi as u32, //4
        divisor_lo as u32, //5
        remainder_hi as u32, //6
        remainder_lo as u32); //7
    WaitMscCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn WaitMscUnchecked<'r> (c : &'r Connection,
                         drawable : xproto::Drawable,
                         target_msc_hi : u32,
                         target_msc_lo : u32,
                         divisor_hi : u32,
                         divisor_lo : u32,
                         remainder_hi : u32,
                         remainder_lo : u32) -> WaitMscCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_wait_msc_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        target_msc_hi as u32, //2
        target_msc_lo as u32, //3
        divisor_hi as u32, //4
        divisor_lo as u32, //5
        remainder_hi as u32, //6
        remainder_lo as u32); //7
    WaitMscCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl WaitMscReply {
  pub fn ust_hi(&mut self) -> u32 {
    unsafe { accessor!(ust_hi -> u32, (*self.base.reply)) }
  }

  pub fn ust_lo(&mut self) -> u32 {
    unsafe { accessor!(ust_lo -> u32, (*self.base.reply)) }
  }

  pub fn msc_hi(&mut self) -> u32 {
    unsafe { accessor!(msc_hi -> u32, (*self.base.reply)) }
  }

  pub fn msc_lo(&mut self) -> u32 {
    unsafe { accessor!(msc_lo -> u32, (*self.base.reply)) }
  }

  pub fn sbc_hi(&mut self) -> u32 {
    unsafe { accessor!(sbc_hi -> u32, (*self.base.reply)) }
  }

  pub fn sbc_lo(&mut self) -> u32 {
    unsafe { accessor!(sbc_lo -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(WaitMscCookie<'s>, mk_reply_xcb_dri2_wait_msc_reply_t, WaitMscReply, xcb_dri2_wait_msc_reply);

pub fn WaitSbc<'r> (c : &'r Connection,
                drawable : xproto::Drawable,
                target_sbc_hi : u32,
                target_sbc_lo : u32) -> WaitSbcCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_wait_sbc(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        target_sbc_hi as u32, //2
        target_sbc_lo as u32); //3
    WaitSbcCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn WaitSbcUnchecked<'r> (c : &'r Connection,
                         drawable : xproto::Drawable,
                         target_sbc_hi : u32,
                         target_sbc_lo : u32) -> WaitSbcCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_wait_sbc_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        target_sbc_hi as u32, //2
        target_sbc_lo as u32); //3
    WaitSbcCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl WaitSbcReply {
  pub fn ust_hi(&mut self) -> u32 {
    unsafe { accessor!(ust_hi -> u32, (*self.base.reply)) }
  }

  pub fn ust_lo(&mut self) -> u32 {
    unsafe { accessor!(ust_lo -> u32, (*self.base.reply)) }
  }

  pub fn msc_hi(&mut self) -> u32 {
    unsafe { accessor!(msc_hi -> u32, (*self.base.reply)) }
  }

  pub fn msc_lo(&mut self) -> u32 {
    unsafe { accessor!(msc_lo -> u32, (*self.base.reply)) }
  }

  pub fn sbc_hi(&mut self) -> u32 {
    unsafe { accessor!(sbc_hi -> u32, (*self.base.reply)) }
  }

  pub fn sbc_lo(&mut self) -> u32 {
    unsafe { accessor!(sbc_lo -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(WaitSbcCookie<'s>, mk_reply_xcb_dri2_wait_sbc_reply_t, WaitSbcReply, xcb_dri2_wait_sbc_reply);

pub fn SwapIntervalChecked<'r> (c : &'r Connection,
                            drawable : xproto::Drawable,
                            interval : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_swap_interval_checked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        interval as u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SwapInterval<'r> (c : &'r Connection,
                     drawable : xproto::Drawable,
                     interval : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_swap_interval(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        interval as u32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetParam<'r> (c : &'r Connection,
                 drawable : xproto::Drawable,
                 param : u32) -> GetParamCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_get_param(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        param as u32); //2
    GetParamCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetParamUnchecked<'r> (c : &'r Connection,
                          drawable : xproto::Drawable,
                          param : u32) -> GetParamCookie<'r> {
  unsafe {
    let cookie = xcb_dri2_get_param_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        param as u32); //2
    GetParamCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetParamReply {
  pub fn is_param_recognized(&mut self) -> u8 {
    unsafe { accessor!(is_param_recognized -> u8, (*self.base.reply)) }
  }

  pub fn value_hi(&mut self) -> u32 {
    unsafe { accessor!(value_hi -> u32, (*self.base.reply)) }
  }

  pub fn value_lo(&mut self) -> u32 {
    unsafe { accessor!(value_lo -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetParamCookie<'s>, mk_reply_xcb_dri2_get_param_reply_t, GetParamReply, xcb_dri2_get_param_reply);


impl BufferSwapCompleteEvent {
  pub fn event_type(&mut self) -> u16 {
    unsafe { accessor!(event_type -> u16, (*self.base.event)) }
  }

  pub fn drawable(&mut self) -> xproto::Drawable {
    unsafe { accessor!(drawable -> xproto::Drawable, (*self.base.event)) }
  }

  pub fn ust_hi(&mut self) -> u32 {
    unsafe { accessor!(ust_hi -> u32, (*self.base.event)) }
  }

  pub fn ust_lo(&mut self) -> u32 {
    unsafe { accessor!(ust_lo -> u32, (*self.base.event)) }
  }

  pub fn msc_hi(&mut self) -> u32 {
    unsafe { accessor!(msc_hi -> u32, (*self.base.event)) }
  }

  pub fn msc_lo(&mut self) -> u32 {
    unsafe { accessor!(msc_lo -> u32, (*self.base.event)) }
  }

  pub fn sbc(&mut self) -> u32 {
    unsafe { accessor!(sbc -> u32, (*self.base.event)) }
  }

  pub fn new(event_type : u16,
         drawable : xproto::Drawable,
         ust_hi : u32,
         ust_lo : u32,
         msc_hi : u32,
         msc_lo : u32,
         sbc : u32) -> BufferSwapCompleteEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_dri2_buffer_swap_complete_event_t;
      (*raw).event_type = event_type;
      (*raw).drawable = drawable;
      (*raw).ust_hi = ust_hi;
      (*raw).ust_lo = ust_lo;
      (*raw).msc_hi = msc_hi;
      (*raw).msc_lo = msc_lo;
      (*raw).sbc = sbc;
      BufferSwapCompleteEvent { base : Event { event : raw as *mut xcb_dri2_buffer_swap_complete_event_t }}
    }
  }
}

impl InvalidateBuffersEvent {
  pub fn drawable(&mut self) -> xproto::Drawable {
    unsafe { accessor!(drawable -> xproto::Drawable, (*self.base.event)) }
  }

  pub fn new(drawable : xproto::Drawable) -> InvalidateBuffersEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_dri2_invalidate_buffers_event_t;
      (*raw).drawable = drawable;
      InvalidateBuffersEvent { base : Event { event : raw as *mut xcb_dri2_invalidate_buffers_event_t }}
    }
  }
}

