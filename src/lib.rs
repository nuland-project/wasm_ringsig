//#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
//#![feature(proc_macro_span)]
//#![feature(proc_macro_raw_ident)]
extern crate wasm_bindgen;
extern crate js_sys;

extern crate fujisaki_ringsig as ringsig;
extern crate hex;
extern crate serde;
extern crate serde_json;

use wasm_bindgen::prelude::*;

use std::mem::transmute;

//#[wasm_bindgen]
//extern {
//    fn alert(s: &str);
//}
//
//#[wasm_bindgen]
//pub fn big_computation() {
//    alert("Big computation in Rust");
//}

// Strings can both be passed in and received
#[wasm_bindgen]
#[no_mangle]
pub extern fn get_str() -> String {
    return "hello from rust".to_owned();
}

// Strings can both be passed in and received
#[wasm_bindgen]
#[no_mangle]
pub extern fn concat_strs_2(pubkeys: js_sys::Set) -> String {
    let mut result = "".to_owned();
    for k in pubkeys.keys() {
        // If `k` is a string, add it to result
//        if k.unwrap().is_string() {
        result = format!("{:?},{:?}", result, k.unwrap());
//        }
    }
    return result;
}

#[wasm_bindgen]
pub fn collect_numbers(some_iterable: &JsValue) -> Result<js_sys::Array, JsValue> {
    let nums = js_sys::Array::new();

    let iterator = js_sys::try_iter(some_iterable)?.unwrap();

    for x in iterator {
        // If the iterator's `next` method throws an error, propagate it
        // up to the caller.
        let x = x?;

        // If `x` is a number, add it to our array of numbers!
        if x.as_f64().is_some() {
            nums.push(&x);
        }
    }

    Ok(nums)
}

#[wasm_bindgen]
#[no_mangle]
pub fn some_sign() -> String {
    sign_rust(
        1u16,
        "question".to_owned(),
        vec![
            "809be5e9bfae20fa72a02a1f025a5649ccf0f920dd43c86dfc7735cdca46f87b".to_owned(),
            "d6127abeab907169705b14db5c8bdcca95d81c4ae9dd30a67aabc061cb3d8621".to_owned(),
        ],
        "e7f014bae11646ce9a35dbd2ebc37bb8e5dbdfd4f3f399c4df4e49c2e8fe320cd6127abeab907169705b14db5c8bdcca95d81c4ae9dd30a67aabc061cb3d8621".to_owned(),
    )
}

#[wasm_bindgen]
#[no_mangle]
/// this function accepts array of strings for 'publeys' var
pub extern fn sign(
    answer: u16,
    question: String,
    pubkeys: &JsValue,  //array of strings is expected
    privkey: String,
//    ) -> String {
) -> Result<JsValue, JsValue> {
    let mut pubkeys_rust: Vec<String> = vec![];
    let iterator = js_sys::try_iter(pubkeys)?.unwrap();

    for pk in iterator {
        // If the iterator's `next` method throws an error, propagate it
        // up to the caller.
        let pk = pk?;

        // todo: If `pk` is not a string, throw error
        if pk.is_string() {
            pubkeys_rust.push(pk.as_string().unwrap());
        }
    }

//    return Ok(JsValue::from(format!("{:?}", pubkeys_rust)));

//    return Ok(JsValue::from(sign_rust(
//        answer,
//        question,
//        pubkeys_rust,
//        privkey,
//    )));

    return Ok(
        JsValue::from(
            sign_rust(
                1u16,
                "question".to_owned(),
                vec![
                    "809be5e9bfae20fa72a02a1f025a5649ccf0f920dd43c86dfc7735cdca46f87b".to_owned(),
                    "d6127abeab907169705b14db5c8bdcca95d81c4ae9dd30a67aabc061cb3d8621".to_owned(),
                ],
                "e7f014bae11646ce9a35dbd2ebc37bb8e5dbdfd4f3f399c4df4e49c2e8fe320cd6127abeab907169705b14db5c8bdcca95d81c4ae9dd30a67aabc061cb3d8621".to_owned(),
            )
        )
    );
}

//#[wasm_bindgen]
//#[no_mangle]
/// this function accepts values of pure rust types
fn sign_rust(
    answer: u16,
    question: String,
    pubkeys: Vec<String>,
    privkey: String,
) -> String {
//    let Context { msg, tag, mut keypairs } = setup(10);
//    let privkey = remove_privkey(&mut keypairs);
    let privkey = ringsig::key::PrivateKey::from_bytes(&*hex::decode(privkey).unwrap()).unwrap();

    let pubkeys: Vec<ringsig::key::PublicKey> = pubkeys.iter().map(|s| ringsig::key::PublicKey::from_bytes(&*hex::decode(s).unwrap()).unwrap()).collect();


    let tag = ringsig::sig::Tag {
        pubkeys: pubkeys,
        issue: Vec::from(question.as_bytes()),
    };


    let answer_bytes: [u8; 2] = unsafe { transmute(answer.to_le()) };


    let sig = ringsig::sig::sign(
        &answer_bytes,
        &tag,
        &privkey,
    );

    return format!("{:?}", sig);
//    println!("verify result:{:?}", ringsig::sig::verify(
//        &answer_bytes,
//        &tag,
//        &sig,
//    ));
//    return format!("{:?}", sig);
}


#[cfg(test)]
mod test {
    // it works
    #[test]
    fn tmp() {
        assert_eq!("hello from rust".to_owned(), super::get_str());
        println!("{}", super::some_sign());
    }

    #[test]
    fn test_sign() {
        println!("sign: {:?}", super::sign_rust(
            1u16,
            "question".to_owned(),
            vec![
                "809be5e9bfae20fa72a02a1f025a5649ccf0f920dd43c86dfc7735cdca46f87b".to_owned(),
                "d6127abeab907169705b14db5c8bdcca95d81c4ae9dd30a67aabc061cb3d8621".to_owned(),
            ],
            "e7f014bae11646ce9a35dbd2ebc37bb8e5dbdfd4f3f399c4df4e49c2e8fe320cd6127abeab907169705b14db5c8bdcca95d81c4ae9dd30a67aabc061cb3d8621".to_owned(),
        ));
    }
}