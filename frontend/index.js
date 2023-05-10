import 'regenerator-runtime/runtime';
import { Wallet } from './near-wallet';
import init, { Grid } from "../wasm-modules/pkg/wasm_modules.js";

const CONTRACT_ADDRESS = process.env.CONTRACT_NAME;
// When creating the wallet you can optionally ask to create an access key
// Having the key enables to call non-payable methods without interrupting the user to sign
const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ADDRESS })
let account = ""
let current = ""
// Setup on page load
window.onload = async () => {
  let isSignedIn = await wallet.startUp();
  if (isSignedIn) {
    signedInFlow();
    account = wallet.accountId
  } else {
    signedOutFlow();
  }
  getToken();
};

// Button clicks
document.querySelector('form').onsubmit = setInfo;
document.querySelector('form').oninput = changeGrid;
document.querySelector('#sign-in-button').onclick = () => { wallet.signIn(); };
document.querySelector('#sign-out-button').onclick = () => { wallet.signOut(); };

function changeGrid(event) {
  current = event.target.value
}

async function setInfo(event) {
  // handle UI
  event.preventDefault();
  const token = event.target.elements.token.value;
  document.querySelector('#signed-in-flow main')
    .classList.add('please-wait');

  // use the wallet to send the greeting to the Smart Contract
  await wallet.callMethod({ method: 'set_info', args: { token_id: token, owner_id: account }, contractId: CONTRACT_ADDRESS });

  // query the new greeting
  await getToken();

  // handle UI stuff
  document.querySelector('#signed-in-flow main').classList.remove('please-wait');
}

async function getToken() {
  // use the wallet to query the Smart Contract
  const currentToken = await wallet.viewMethod({ method: 'get_token', args: { owner_id: account }, contractId: CONTRACT_ADDRESS });

  // handle UI stuff
  document.querySelectorAll('[data-behavior=token]').forEach(el => {
    el.innerText = currentToken;
    el.value = currentToken;
  });
}

// UI: Display the signed-out-flow container
function signedOutFlow() {
  document.querySelector('#signed-in-flow').style.display = 'none';
  document.querySelector('#signed-out-flow').style.display = 'block';
}

// UI: Displaying the signed in flow container and fill in account-specific data
function signedInFlow() {
  document.querySelector('#signed-out-flow').style.display = 'none';
  document.querySelector('#signed-in-flow').style.display = 'block';
  document.querySelectorAll('[data-behavior=account-id]').forEach(el => {
    el.innerText = wallet.accountId;
  });
}

init().then(() => {
  const pre = document.getElementById("wasm-render");
  const grid = Grid.new()

  const renderLoop = () => {
    pre.textContent = ""
    pre.textContent = grid.render();
    grid.tick(current);
    requestAnimationFrame(renderLoop);
  };

  requestAnimationFrame(renderLoop);
})
