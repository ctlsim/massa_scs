import init, { read_wasm } from './sc_scan_01.js';


const JSONRPC_URL = "https://buildnet.massa.net/api/v2";
const CHAIN_ID = 77658366n;
const WALLET_SECRET_KEY = "";
let init_ = false;
let wasmParseResultElement = null;

async function debug() {

    const requestHeaders = {
        Accept:
            'application/json,text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
            'Access-Control-Allow-Origin': '*',
            'Access-Control-Allow-Credentials': true,
            'Access-Control-Allow-Methods': 'GET,PUT,POST,DELETE,PATCH,OPTIONS',
    } // as AxiosRequestHeaders;

    const data = [];
    data.push({
        address: "AS12UJM8JYR6cRzXgkUGTjRDu9hicJUPkDvy7zGhGZq3WgZ511dwR",
        is_final: true,
    });
    const body = {
        jsonrpc: '2.0',
        method: 'get_addresses_bytecode',
        params: [data],
        id: 0,
    }

    const resp = await axios.post(
        JSONRPC_URL,
        body,
        requestHeaders
    );

    // console.log("resp:", resp);

    let b = Uint8Array.from(resp.data.result[0]);
    // console.log("b:", b);
    // let b = new Uint8Array(0);

    await init();
    let res = read_wasm(b);
    console.log("wasm parse:", res);
}

// debug();

async function parseFromAddress(address) {

    const requestHeaders = {
        Accept:
            'application/json,text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Credentials': true,
        'Access-Control-Allow-Methods': 'GET,PUT,POST,DELETE,PATCH,OPTIONS',
    } // as AxiosRequestHeaders;

    const data = [];
    data.push({
        address: address,
        is_final: true,
    });
    const body = {
        jsonrpc: '2.0',
        method: 'get_addresses_bytecode',
        params: [data],
        id: 0,
    }

    const resp = await axios.post(
        JSONRPC_URL,
        body,
        requestHeaders
    );

    console.log("resp", resp);

    let b = Uint8Array.from(resp.data.result[0]);

    if (init_ === false) {
        await init();
        init_ = true;
    }
    let res = read_wasm(b);
    console.log("wasm parse:", res);

    return JSON.parse(res);
}

async function parseForm() {

    var el = document.getElementById("input_address");
    let input_sc_address = el.value;
    console.log("Input address", input_sc_address);

    if (input_sc_address !== undefined && input_sc_address !== null && input_sc_address !== "" || input_sc_address.length == 32) {
        let json_res = await parseFromAddress(input_sc_address);
        console.log("json res:", json_res);
        console.log("===");
        /*
        for (let obj of json_res) {
            console.log("obj:", obj);
        }
        */

        if (wasmParseResultElement !== null) {
            wasmParseResultElement.remove();
        }

        var newDiv = document.createElement('div');
        var newP1 = document.createElement('p');
        newP1.appendChild(document.createTextNode(`SC address: ${input_sc_address}`));
        newDiv.appendChild(newP1);
        var newList = document.createElement('ul');
        newDiv.appendChild(newList);
        for (let obj of json_res) {
            if (obj.type === "Version") {
                var newListItem = document.createElement("li");
                var newContent = document.createTextNode(`${obj.type}: ${obj.num}\n`);
                newListItem.appendChild(newContent);
                newList.appendChild(newListItem);
            } else if (obj.type === "Import") {
                var newListItem = document.createElement("li");
                var newContent = document.createTextNode(`${obj.type}: ${obj.module}.${obj.name}\n`);
                newListItem.appendChild(newContent);
                newList.appendChild(newListItem);
            } else if (obj.type === "Export") {
                var newListItem = document.createElement("li");
                var newContent = document.createTextNode(`${obj.type} - ${obj.kind}: ${obj.name}\n`);
                newListItem.appendChild(newContent);
                newList.appendChild(newListItem);
            }
        }

        wasmParseResultElement = newDiv;
        document.body.append(newDiv);
    }
}

// bind button click to function
document.getElementById("parse").onclick = parseForm;
