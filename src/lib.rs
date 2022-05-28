extern crate proc_macro;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_uint;
use std::str::FromStr;
use proc_macro2::TokenStream;
use syn::parse_macro_input;
use std::path::Path;
use quote::{quote, ToTokens};
use stereokit_rs::enums::{DepthMode, DisplayBlend, DisplayMode, LogFilter};
use syn::__private::TokenStream2;


#[proc_macro]
pub fn create_sk_settings(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let my_str = _item.to_string();
    let mut my_map = HashMap::new();
    my_map.insert("app_name", SkSettingsEnum::String("default"));
    my_map.insert("assets_folder", SkSettingsEnum::Path(&Path::new("./")));
    my_map.insert("display_preference", SkSettingsEnum::DisplayMode(DisplayMode::MixedReality));
    my_map.insert("blend_preference", SkSettingsEnum::DisplayBlend(DisplayBlend::None));
    my_map.insert("depth_mode", SkSettingsEnum::DepthMode(DepthMode::Balanced));
    my_map.insert("log_filter", SkSettingsEnum::LogFilter(LogFilter::None));
    my_map.insert("no_flatscreen_fallback", SkSettingsEnum::Boolean(false));
    my_map.insert("overlay_app", SkSettingsEnum::Boolean(false));
    my_map.insert("overlay_priority", SkSettingsEnum::UnsignedInteger(0));
    my_map.insert("flatscreen_pos_x", SkSettingsEnum::Integer(0));
    my_map.insert("flatscreen_pos_y", SkSettingsEnum::Integer(0));
    my_map.insert("flatscreen_width", SkSettingsEnum::Integer(400));
    my_map.insert("flatscreen_height", SkSettingsEnum::Integer(400));
    my_map.insert("disable_unfocused_sleep", SkSettingsEnum::Boolean(false));
    my_map.insert("disable_flatscreen_mr_sim", SkSettingsEnum::Integer(0));

    // my_str.split(",").map(|split_str_1| {
    // 	split_str_1.split(":")
    // });
    let mut first = quote! {

	};
    for my_str in my_map {
        let the_str = TokenStream2::from_str(my_str.0).unwrap();
        let settings_enum = my_str.1;
        match settings_enum {
            SkSettingsEnum::String(s) => {
                first = quote! {
					#first
					#the_str: std::ffi::CString::new(#s).unwrap().as_ptr(),
				}
            }
            SkSettingsEnum::Boolean(b) => {
                let mut x: i32 = 0;
                if(b) {
                    x = 1;
                }
                first = quote! {
					#first
					#the_str: #x,
				}
            }
            SkSettingsEnum::Path(p) => {
                let str = p.to_str().unwrap();
                first = quote! {
					#first
					#the_str: std::ffi::CString::new(#str).unwrap().as_ptr(),
				}
            }
            SkSettingsEnum::Integer(i) => {
                let x: i32 = i;
                first = quote! {
					#first
					#the_str: #x,
				}
            }
            SkSettingsEnum::UnsignedInteger(i) => {
                let x: u32 = i;
                first = quote! {
					#first
					#the_str: #x,
				}
            }
            SkSettingsEnum::DisplayMode(d) => {
                let x = std::os::raw::c_uint::from(d as u32);
                first = quote! {
					#first
					#the_str: #x,
				}
            }
            SkSettingsEnum::DisplayBlend(d) => {
                let x = std::os::raw::c_uint::from(d as u32);
                first = quote! {
					#first
					#the_str: #x,
				}
            }
            SkSettingsEnum::DepthMode(d) => {
                let x = std::os::raw::c_uint::from(d as u32);
                first = quote! {
					#first
					#the_str: #x,
				}
            }
            SkSettingsEnum::LogFilter(l) => {
                let x = std::os::raw::c_uint::from(l as u32);
                first = quote! {
					#first
					#the_str: #x,
				}
            }
        };
    }
    first = quote! {
		SKSettings {
			#first
			android_java_vm: core::ptr::null_mut(),
			android_activity: core::ptr::null_mut(),
		}
	};
    first.into()
}

enum SkSettingsEnum<'a> {
    String(&'a str),
    Boolean(bool),
    Path(&'a Path),
    Integer(i32),
    UnsignedInteger(u32),
    DisplayMode(DisplayMode),
    DisplayBlend(DisplayBlend),
    DepthMode(DepthMode),
    LogFilter(LogFilter),
}
