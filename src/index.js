import React from 'react';
import ReactDOM from 'react-dom';

// const { get_str } = wasm_bindgen;
//
// function run(){
//     console.log(get_str());
// }
//
// wasm_bindgen('../build/react_rust_wasm').then(run);

const wasm = import("../build/react_rust_wasm");
console.log(wasm);

function parseHexString(str) {
    var result = [];
    while (str.length >= 8) {
        result.push(parseInt(str.substring(0, 8), 16));

        str = str.substring(8, str.length);
    }

    return result;
}

wasm.then(wasm => {
    console.log(wasm);
    console.log(wasm.get_str);
    console.log(wasm.get_str());

    console.log("try pass set of numbers to collect_numbers:");
    console.log(wasm.collect_numbers([1, 2]));

//    console.log("try pass set of strs to get_str2:");
//    console.log(wasm.concat_strs_2(["1", "2"]));

    console.log("try print some sign:");
    console.log(wasm.some_sign());

    console.log("try to sign:");
    console.log(wasm.sign(
        1,
        "survey_title",
        [
            "809be5e9bfae20fa72a02a1f025a5649ccf0f920dd43c86dfc7735cdca46f87b",
            "d6127abeab907169705b14db5c8bdcca95d81c4ae9dd30a67aabc061cb3d8621"
        ],
        "e7f014bae11646ce9a35dbd2ebc37bb8e5dbdfd4f3f399c4df4e49c2e8fe320cd6127abeab907169705b14db5c8bdcca95d81c4ae9dd30a67aabc061cb3d8621"
    ));

  //
  // const App = () => {
  //   return (
  //     <div>
  //       <h1>Hi there</h1>
  //       <button onClick={wasm.big_computation}>Run Computation</button>
  //     </div>
  //   );
  // }
  //
  // ReactDOM.render(
  //   <App />,
  //   // document.getElementById('root')
  //   document.body
  // );
});

