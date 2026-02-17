import init, { Transaction, calculate_change } from "../pkg/changer_wasm.js"

await init();

function setTextContent(element_id, text) {
  document.getElementById(element_id).textContent = text;
}

function addInnerHtml(element_id, html) {
  document.getElementById(element_id).innerHTML += html;
}

function refresh(result) {
  const bills = document.getElementById("bills")
  const coins = document.getElementById("coins")
  bills.innerHTML = "";
  coins.innerHTML = "";

  let bill_order = ["hundreds", "fifties", "twenties", "tens", "fives", "ones"]
  let bill_order_id = 0;

  let coin_order = ["quarters", "dimes", "nickels", "pennies"]
  let coin_order_id = 0;

  while (true) {
    if (bill_order_id == bill_order.length) break;

    for (const [name, arr] of Object.entries(result.bills)) {
      if (name == bill_order[bill_order_id]) {
        let count = arr[0]
        let value = arr[1]
        for (let i = 0; i < count; i++) {
          const img = document.createElement("img");
          img.setAttribute("src", `currencies/${name}.png`);
          img.id = value;
          img.width = 261;
          img.height = 100;
          bills.appendChild(img);
        }
        bill_order_id++;
      }
    }
  }

  while (true) {
    if (coin_order_id == coin_order.length) break;

    for (const [name, arr] of Object.entries(result.coins)) {
      if (name == coin_order[coin_order_id]) {
        let count = arr[0]
        let value = arr[1]
        for (let i = 0; i < count; i++) {
          let size = 50;
          if (name === "dimes" || name === "pennies") size *= 0.3;
          if (name === "nickels") size *= 0.9;

          const img = document.createElement("img");
          img.setAttribute("src", `currencies/${name}.png`);
          img.id = value;
          img.width = size;
          img.height = size;
          coins.appendChild(img)
        }
        coin_order_id++;
      }
      addInnerHtml("coins", "<br>");
    }
  }
}

document.getElementById("calculate").onclick = () => {
  const cost = parseFloat(document.getElementById("cost_input").value);
  const given = parseFloat(document.getElementById("given_input").value);

  const transaction = new Transaction(cost, given);

  let calculated = JSON.parse(calculate_change(transaction));

  refresh(calculated);

};
