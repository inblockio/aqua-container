

export const SEPOLIA_SMART_CONTRACT_ADDRESS = "0x45f59310ADD88E6d23ca58A0Fa7A55BEE6d2a611"

// export const await API_ENDPOINT() =  //import.meta.env.VITE_API_ENDPOINT

// export const await API_ENDPOINT() = `http://${import.meta.env.VITE_REMOTE || '127.0.0.1'}:${ import.meta.env.VITE_REMOTE_PORT || 3600}`;

export const SESSION_COOKIE_NAME = "pkc_nonce"

export const ETH_CHAINID_MAP: Record<string, string> = {
  'mainnet': '0x1',
  'sepolia': '0xaa36a7',
  'holesky': '0x4268',
}

export const ETH_CHAIN_ADDRESSES_MAP: Record<string, string> = {
  'mainnet': '0x45f59310ADD88E6d23ca58A0Fa7A55BEE6d2a611',
  'sepolia': '0x45f59310ADD88E6d23ca58A0Fa7A55BEE6d2a611',
  'holesky': '0x45f59310ADD88E6d23ca58A0Fa7A55BEE6d2a611',
}

export const WITNESS_NETWORK_MAP: Record<string, string> = {
  'mainnet': 'https://etherscan.io/tx',
  'sepolia': 'https://sepolia.etherscan.io/tx',
  'holesky': 'https://holesky.etherscan.io/tx',
}


export const imageTypes = ["image/png", "image/jpeg", "image/gif", "image/svg+xml"];
export const documentTypes = ["application/pdf", "application/msword", "application/vnd.openxmlformats-officedocument.wordprocessingml.document"];
export const musicTypes = ["audio/mpeg", "audio/wav"];
export const videoTypes = ["video/mp4", "video/mpeg", "video/webm"];





// Function to initialize the backend URL
export const initializeBackendUrl = async (): Promise<string> => {
  let BACKEND_URL = "http://127.0.0.1:3600";
  try {
    // Fetch the config.json file from the public folder
    const response = await fetch('/config.json');

    // Check if the response is successful
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    // Parse the JSON
    const configData = await response.json();

    console.log("Data from config ", configData);
    // Update the BACKEND_URL
    BACKEND_URL = configData.BACKEND_URL || "http://127.0.0.1:3600";
    if (BACKEND_URL == "BACKEND_URL_PLACEHOLDER"){
      BACKEND_URL="http://127.0.0.1:3600";
    }
    console.log("Config Backend URL", BACKEND_URL);
  } catch (err) {
    // If there's an error, it will use the default URL
    console.error('Error reading config:', err);
  }

  return BACKEND_URL;
};


// const BACKEND_URL = "0.0.0.0.0";
// Generate endpoints function
// export const ENDPOINTS = () => ({
  // SIWE_SIGN_IN: `${BACKEND_URL}/siwe`,
  // FETCH_ADDRESS_BY_NONCE: `${BACKEND_URL}/fetch_nonce_session`,
  // SIWE_SIGN_OUT: `${BACKEND_URL}/siwe_logout`,

  // FETCH_USER_PROFILE: `${BACKEND_URL}/explorer_fetch_user_profile`,
  // UPDATE_USER_PROFILE: `${BACKEND_URL}/explorer_update_user_profile`,

 // EXPOLORER_FETCH_FILES: `${BACKEND_URL}/explorer_files`,
 // SIGN_FILE: `${BACKEND_URL}/explorer_sign_revision`,
  // WITNESS_FILE: `${BACKEND_URL}/explorer_witness_file`,
  // DELETE_FILE: `${BACKEND_URL}/explorer_delete_file`,
  // UPLOAD_FILE: `${BACKEND_URL}/explorer_file_upload`,
  // IMPORT_AQUA_CHAIN: `${BACKEND_URL}/explorer_aqua_file_upload`,
  // DELETE_ALL_FILES: `${BACKEND_URL}/explorer_delete_all_files`,
// });
