

export const SEPOLIA_SMART_CONTRACT_ADDRESS = "0x45f59310ADD88E6d23ca58A0Fa7A55BEE6d2a611"

// export const await API_ENDPOINT() =  //import.meta.env.VITE_API_ENDPOINT

// export const await API_ENDPOINT() = `http://${import.meta.env.VITE_REMOTE || '127.0.0.1'}:${ import.meta.env.VITE_REMOTE_PORT || 3600}`;

export const API_ENDPOINT = async (): Promise<string> => {
  let url = "";
  try {
    // Fetch the config.json file from the public folder
    const response = await fetch('/config.json');

    // Check if the response is successful
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    // Parse the JSON
    const configData = await response.json();

    // Update state with the config data
    console.log("Config ", configData);
    url = configData.BACKEND_URL
  } catch (err) {
    // Handle any errors that occur during fetching or parsing
    url = "http://127.0.0.1:3000"
    console.error('Error reading config:', err);
  }
  return url;
}
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



export const ENDPOINTS = {
  SIWE_SIGN_IN: `${await API_ENDPOINT()}/siwe`,
  FETCH_ADDRESS_BY_NONCE: `${await API_ENDPOINT()}/fetch_nonce_session`,
  SIWE_SIGN_OUT: `${await API_ENDPOINT()}/siwe_logout`,

  // FETCH_CONFIGURATION: `${await API_ENDPOINT()}/explorer_fetch_configuration`,
  // UPDATE_CONFIGURATION: `${await API_ENDPOINT()}/explorer_update_configuration`,

  FETCH_USER_PROFILE: `${await API_ENDPOINT()}/explorer_fetch_user_profile`,
  UPDATE_USER_PROFILE: `${await API_ENDPOINT()}/explorer_update_user_profile`,

  EXPOLORER_FETCH_FILES: `${await API_ENDPOINT()}/explorer_files`,
  SIGN_FILE: `${await API_ENDPOINT()}/explorer_sign_revision`,
  WITNESS_FILE: `${await API_ENDPOINT()}/explorer_witness_file`,
  DELETE_FILE: `${await API_ENDPOINT()}/explorer_delete_file`,
  UPLOAD_FILE: `${await API_ENDPOINT()}/explorer_file_upload`,
  IMPORT_AQUA_CHAIN: `${await API_ENDPOINT()}/explorer_aqua_file_upload`,
  DELETE_ALL_FILES: `${await API_ENDPOINT()}/explorer_delete_all_files`,
}

export const imageTypes = ["image/png", "image/jpeg", "image/gif", "image/svg+xml"];
export const documentTypes = ["application/pdf", "application/msword", "application/vnd.openxmlformats-officedocument.wordprocessingml.document"];
export const musicTypes = ["audio/mpeg", "audio/wav"];
export const videoTypes = ["video/mp4", "video/mpeg", "video/webm"];