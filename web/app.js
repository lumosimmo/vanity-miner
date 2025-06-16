import init, {
  wasm_mine_create2_salt_with_prefix,
  wasm_mine_create2_salt_with_suffix,
  wasm_mine_create2_salt_with_contains,
  wasm_mine_create3_salt_with_prefix,
  wasm_mine_create3_salt_with_suffix,
  wasm_mine_create3_salt_with_contains,
  wasm_mine_eulerswap_salt,
  wasm_mine_v4_hook_salt,
} from "../pkg/vanity_miner.js";

let isInitialized = false;

function hexToBytes(hex) {
  if (hex.startsWith("0x")) {
    hex = hex.slice(2);
  }
  if (hex.length % 2 !== 0) {
    hex = "0" + hex;
  }
  const bytes = new Uint8Array(hex.length / 2);
  for (let i = 0; i < hex.length; i += 2) {
    bytes[i / 2] = parseInt(hex.substr(i, 2), 16);
  }
  return bytes;
}

function parseSeed(seedInput) {
  if (!seedInput) return null;

  const seed = seedInput.startsWith("0x")
    ? parseInt(seedInput, 16)
    : parseInt(seedInput, 10);

  return isNaN(seed) ? false : seed;
}

function getInputValue(id) {
  return document.getElementById(id).value;
}

function getInputValueTrimmed(id) {
  return document.getElementById(id).value.trim();
}

function getInputNumber(id) {
  return parseInt(document.getElementById(id).value);
}

function getCheckedRadioValue(name) {
  return document.querySelector(`input[name="${name}"]:checked`).value;
}

async function measureTime(fn) {
  const startTime = performance.now();
  const result = await fn();
  const endTime = performance.now();
  const elapsedTime = Math.round(endTime - startTime);
  return { result, elapsedTime };
}

async function initWasm() {
  if (!isInitialized) {
    await init();
    isInitialized = true;
    console.log("WASM module initialized");
  }
}

function showError(message) {
  const errorDiv = document.getElementById("error");
  errorDiv.textContent = message;
  errorDiv.style.display = "block";
  setTimeout(() => {
    errorDiv.style.display = "none";
  }, 5000);
}

function displayResults(resultObject, type, elapsedTime) {
  const resultsDiv = document.getElementById("results");
  const resultsContent = document.getElementById("results-content");
  resultsDiv.style.display = "block";

  resultsContent.innerHTML = "";

  const results = resultObject.results || [];
  const totalIterations = resultObject.total_iterations || 0;

  const iterationsPerSecond =
    elapsedTime > 0 ? Math.round(totalIterations / (elapsedTime / 1000)) : 0;

  if (!results || results.length === 0) {
    resultsContent.innerHTML = `<p>No results found in ${elapsedTime}ms (${totalIterations.toLocaleString()} iterations, ${iterationsPerSecond.toLocaleString()} it/s). Try increasing max iterations or using a shorter pattern.</p>`;
    return;
  }

  resultsContent.innerHTML = `<p><strong>Found ${
    results.length
  } results in ${elapsedTime}ms (${totalIterations.toLocaleString()} iterations, ${iterationsPerSecond.toLocaleString()} it/s)</strong></p>`;

  results.forEach((result) => {
    const item = document.createElement("div");
    item.className = "result-item";

    const addressLabel = type === "eulerswap" ? "Hook Address" : "Address";
    item.innerHTML = `
      <div>${addressLabel}: <span class="address">${result.computed_address}</span></div>
      <div>Salt: <span class="salt">${result.salt}</span></div>
    `;

    if (result.guarded_salt) {
      item.innerHTML += `<div>Guarded Salt: <span class="salt">${result.guarded_salt}</span></div>`;
    }

    resultsContent.appendChild(item);
  });
}

function switchTab(tabName) {
  document
    .querySelectorAll(".tab")
    .forEach((tab) => tab.classList.remove("active"));
  document
    .querySelectorAll(".tab-content")
    .forEach((content) => content.classList.remove("active"));

  document.querySelector(`.tab[data-tab="${tabName}"]`).classList.add("active");
  document.getElementById(tabName).classList.add("active");
}

async function mineWithPattern(config, pattern, patternType, wasmFunctions) {
  const patternBytes = hexToBytes(pattern);

  const wasmFn = wasmFunctions[patternType];
  if (!wasmFn) {
    throw new Error(`Invalid pattern type: ${patternType}`);
  }

  return await wasmFn(config, patternBytes);
}

function validateAndParseSeed(seedInput) {
  const seed = parseSeed(seedInput);
  if (seed === false) {
    showError("Invalid seed value. Please enter a valid number.");
    return { isValid: false };
  }
  return { isValid: true, seed };
}

async function mineCreate2() {
  try {
    await initWasm();

    const deployer = getInputValue("create2-deployer");
    const initCodeHash = getInputValue("create2-init-code-hash");
    const pattern = getInputValue("create2-pattern");
    const patternType = getCheckedRadioValue("create2-pattern-type");
    const maxIterations = getInputNumber("create2-max-iterations");
    const maxResults = getInputNumber("create2-max-results");
    const seedInput = getInputValueTrimmed("create2-seed");

    if (!deployer || !initCodeHash || !pattern) {
      showError("Please fill in all required fields");
      return;
    }

    const { isValid, seed } = validateAndParseSeed(seedInput);
    if (!isValid) return;

    const config = {
      deployer,
      init_code_hash: initCodeHash,
      max_iterations: maxIterations,
      max_results: maxResults,
      seed: seed,
    };

    const wasmFunctions = {
      prefix: wasm_mine_create2_salt_with_prefix,
      suffix: wasm_mine_create2_salt_with_suffix,
      contains: wasm_mine_create2_salt_with_contains,
    };

    const { result: results, elapsedTime } = await measureTime(() =>
      mineWithPattern(config, pattern, patternType, wasmFunctions)
    );

    displayResults(results, "create2", elapsedTime);
  } catch (error) {
    showError(`Error: ${error.message}`);
  }
}

async function mineCreate3() {
  try {
    await initWasm();

    const deployer = getInputValue("create3-deployer");
    const caller = getInputValue("create3-caller") || null;
    const chainIdValue = getInputValue("create3-chain-id");
    const chainId = chainIdValue ? parseInt(chainIdValue) : null;
    const pattern = getInputValue("create3-pattern");
    const patternType = getCheckedRadioValue("create3-pattern-type");
    const maxIterations = getInputNumber("create3-max-iterations");
    const maxResults = getInputNumber("create3-max-results");
    const seedInput = getInputValueTrimmed("create3-seed");

    if (!deployer || !pattern) {
      showError("Please fill in all required fields");
      return;
    }

    const { isValid, seed } = validateAndParseSeed(seedInput);
    if (!isValid) return;

    const config = {
      deployer,
      caller,
      chain_id: chainId,
      max_iterations: maxIterations,
      max_results: maxResults,
      seed: seed,
    };

    const wasmFunctions = {
      prefix: wasm_mine_create3_salt_with_prefix,
      suffix: wasm_mine_create3_salt_with_suffix,
      contains: wasm_mine_create3_salt_with_contains,
    };

    const { result: results, elapsedTime } = await measureTime(() =>
      mineWithPattern(config, pattern, patternType, wasmFunctions)
    );

    displayResults(results, "create3", elapsedTime);
  } catch (error) {
    showError(`Error: ${error.message}`);
  }
}

async function mineEulerSwap() {
  try {
    await initWasm();

    const factory = getInputValue("eulerswap-factory");
    const eulerswapImpl = getInputValue("eulerswap-impl");
    const vault0 = getInputValue("eulerswap-vault0");
    const vault1 = getInputValue("eulerswap-vault1");
    const eulerAccount = getInputValue("eulerswap-euler-account");
    const protocolFeeRecipient = getInputValue(
      "eulerswap-protocol-fee-recipient"
    );
    const equilibriumReserve0 = getInputValue("eulerswap-equilibrium-reserve0");
    const equilibriumReserve1 = getInputValue("eulerswap-equilibrium-reserve1");
    const priceX = getInputValue("eulerswap-price-x");
    const priceY = getInputValue("eulerswap-price-y");
    const concentrationX = getInputValue("eulerswap-concentration-x");
    const concentrationY = getInputValue("eulerswap-concentration-y");
    const fee = getInputValue("eulerswap-fee");
    const protocolFee = getInputValue("eulerswap-protocol-fee");
    const maxIterations = getInputNumber("eulerswap-max-iterations");
    const maxResults = getInputNumber("eulerswap-max-results");
    const seedInput = getInputValueTrimmed("eulerswap-seed");

    if (
      !factory ||
      !eulerswapImpl ||
      !vault0 ||
      !vault1 ||
      !eulerAccount ||
      !protocolFeeRecipient
    ) {
      showError("Please fill in all required fields");
      return;
    }

    const { isValid, seed } = validateAndParseSeed(seedInput);
    if (!isValid) return;

    const config = {
      factory,
      eulerswap_impl: eulerswapImpl,
      pool_params: {
        vault0,
        vault1,
        euler_account: eulerAccount,
        equilibrium_reserve0: equilibriumReserve0.toString(),
        equilibrium_reserve1: equilibriumReserve1.toString(),
        price_x: priceX.toString(),
        price_y: priceY.toString(),
        concentration_x: concentrationX.toString(),
        concentration_y: concentrationY.toString(),
        fee: fee.toString(),
        protocol_fee: protocolFee.toString(),
        protocol_fee_recipient: protocolFeeRecipient,
      },
      max_iterations: maxIterations,
      max_results: maxResults,
      seed: seed,
    };

    const { result: results, elapsedTime } = await measureTime(() =>
      wasm_mine_eulerswap_salt(config)
    );

    displayResults(results, "eulerswap", elapsedTime);
  } catch (error) {
    showError(`Error: ${error.message}`);
  }
}

async function mineV4Hook() {
  try {
    await initWasm();

    const deployer = getInputValue("v4hook-deployer");
    const initCodeHash = getInputValue("v4hook-init-code-hash");
    const maxIterations = getInputNumber("v4hook-max-iterations");
    const maxResults = getInputNumber("v4hook-max-results");
    const seedInput = getInputValueTrimmed("v4hook-seed");

    if (!deployer || !initCodeHash) {
      showError("Please fill in all required fields");
      return;
    }

    const { isValid, seed } = validateAndParseSeed(seedInput);
    if (!isValid) return;

    const permissions = {
      before_initialize: document.getElementById("before-initialize").checked,
      after_initialize: document.getElementById("after-initialize").checked,
      before_add_liquidity: document.getElementById("before-add-liquidity")
        .checked,
      after_add_liquidity: document.getElementById("after-add-liquidity")
        .checked,
      before_remove_liquidity: document.getElementById(
        "before-remove-liquidity"
      ).checked,
      after_remove_liquidity: document.getElementById("after-remove-liquidity")
        .checked,
      before_swap: document.getElementById("before-swap").checked,
      after_swap: document.getElementById("after-swap").checked,
      before_donate: document.getElementById("before-donate").checked,
      after_donate: document.getElementById("after-donate").checked,
      before_swap_return_delta: document.getElementById(
        "before-swap-returns-delta"
      ).checked,
      after_swap_return_delta: document.getElementById(
        "after-swap-returns-delta"
      ).checked,
      after_add_liquidity_return_delta: document.getElementById(
        "after-add-liquidity-returns-delta"
      ).checked,
      after_remove_liquidity_return_delta: document.getElementById(
        "after-remove-liquidity-returns-delta"
      ).checked,
    };

    const config = {
      deployer,
      permissions,
      init_code_hash: initCodeHash,
      max_iterations: maxIterations,
      max_results: maxResults,
      seed: seed,
    };

    const { result: results, elapsedTime } = await measureTime(() =>
      wasm_mine_v4_hook_salt(config)
    );

    displayResults(results, "v4hook", elapsedTime);
  } catch (error) {
    console.error("V4 Hook mining error:", error);
    showError(`Error: ${error.message || error.toString()}`);
  }
}

function generateRandomSeed(inputId) {
  const rand32 = () => BigInt(Math.floor(Math.random() * 0xffffffff));
  const randomSeed =
    (rand32() << 96n) | (rand32() << 64n) | (rand32() << 32n) | rand32();

  document.getElementById(inputId).value = randomSeed.toString();
}

window.generateRandomSeed = generateRandomSeed;

function initTheme() {
  function getThemePreference() {
    const savedTheme = localStorage.getItem("theme");
    if (savedTheme) {
      return savedTheme;
    }

    if (
      window.matchMedia &&
      window.matchMedia("(prefers-color-scheme: dark)").matches
    ) {
      return "dark";
    }

    return "light";
  }

  function applyTheme(theme) {
    if (theme === "dark") {
      document.documentElement.setAttribute("data-theme", "dark");
    } else {
      document.documentElement.removeAttribute("data-theme");
    }

    localStorage.setItem("theme", theme);
  }

  function toggleTheme() {
    const currentTheme = document.documentElement.hasAttribute("data-theme")
      ? "dark"
      : "light";
    const newTheme = currentTheme === "dark" ? "light" : "dark";
    applyTheme(newTheme);
  }

  const initialTheme = getThemePreference();
  applyTheme(initialTheme);

  const themeSwitcher = document.getElementById("theme-switcher");
  if (themeSwitcher) {
    themeSwitcher.addEventListener("click", toggleTheme);
  }

  if (window.matchMedia) {
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", (e) => {
        if (!localStorage.getItem("theme")) {
          applyTheme(e.matches ? "dark" : "light");
        }
      });
  }
}

document.addEventListener("DOMContentLoaded", () => {
  initTheme();

  document.querySelectorAll(".tab").forEach((tab) => {
    tab.addEventListener("click", () => {
      switchTab(tab.getAttribute("data-tab"));
    });
  });

  const buttonMappings = {
    "mine-create2-btn": mineCreate2,
    "mine-create3-btn": mineCreate3,
    "mine-eulerswap-btn": mineEulerSwap,
    "mine-v4hook-btn": mineV4Hook,
  };

  Object.entries(buttonMappings).forEach(([buttonId, handler]) => {
    const button = document.getElementById(buttonId);
    if (button) {
      button.addEventListener("click", handler);
    }
  });

  initWasm().catch(console.error);
});
