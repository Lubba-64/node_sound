


/*
function touchHandler(event)
{
    var touches = event.changedTouches,
        first = touches[0],
        type = "";
    switch(event.type)
    {
        case "touchstart": type = "mousedown"; break;
        case "touchmove":  type = "mousemove"; break;        
        case "touchend":   type = "mouseup";   break;
        default:           return;
    }

    // initMouseEvent(type, canBubble, cancelable, view, clickCount, 
    //                screenX, screenY, clientX, clientY, ctrlKey, 
    //                altKey, shiftKey, metaKey, button, relatedTarget);

    var simulatedEvent = document.createEvent("MouseEvent");
    simulatedEvent.initMouseEvent(type, true, true, window, 1, 
                                  first.screenX, first.screenY, 
                                  first.clientX, first.clientY, false, 
                                  false, false, false, 0/*left*/, null);

    first.target.dispatchEvent(simulatedEvent);
    event.preventDefault();
}

function init() 
{
    document.addEventListener("touchstart", touchHandler, true);
    document.addEventListener("touchmove", touchHandler, true);
    document.addEventListener("touchend", touchHandler, true);
    document.addEventListener("touchcancel", touchHandler, true);    
}
*/

// use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use web_sys::{Document, Event, MouseEvent, TouchEvent, Window};

#[cfg(target_arch = "wasm32")]
fn add_touch_event_listevers() {

    let cb = Closure::wrap(Box::new(|e: TouchEvent| {
        e.prevent_default();
        let mouse_event = touch_to_mouse_event(&e);

        // 
    
        console_log!("{:?}", input.value());
    }) as Box<dyn FnMut(_)>);
    
    input.add_event_listener_with_callback("input", &cb.as_ref().unchecked_ref())?;
    
    cb.forget();
}

#[cfg(target_arch = "wasm32")]
fn touch_to_mouse_event(touch_event: &TouchEvent) -> Option<MouseEvent> {
    let mouse_type = match touch_event.type_().as_str() {
        "touchstart" => "mousedown",
        "touchmove" => "mousemove",
        "touchend" => "mouseup",
        _ => {
            return None;
        }
    };
    let changed_touches = touch_event.changed_touches();
    let first_touch = changed_touches.item(0).unwrap();
    let click_num = changed_touches.length();

    let mut mouse_event = MouseEvent::new(mouse_type).expect("wasm bindgen failed");
    mouse_event.init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_and_related_target_arg(
        mouse_type,
        true,
        true, 
        None,
        1,
        first_touch.screen_x(),
        first_touch.screen_y(),
        first_touch.client_x(),
        first_touch.client_y(),
        false,
        false,
        false,
        false,
        click_num.try_into().unwrap(),
        None,
    );
    Some(mouse_event)
}
fn test() {

}
