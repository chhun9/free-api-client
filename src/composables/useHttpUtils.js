import { invoke } from "@tauri-apps/api/core";
export function useHttpUtils() {
  const getMethodColor = (method) => {
    switch (method) {
      case "GET":
        return "#5692b3";
      case "POST":
        return "#679a73";
      case "PUT":
        return "#7dcdcb";
      case "PATCH":
        return "#51988f";
      case "DELETE":
        return "#c56767";
      default:
        return "#888";
    }
  };

  const getAllMethods = async () => {
    try {
      const methods = await invoke("get_http_methods");
      return methods;
    } catch (error) {
      console.error("Failed to fetch methods:", error);
      return [];
    }
  };

  const getAllHeaders = async () => {
    try {
      const headers = await invoke("get_http_headers");
      return headers;
    } catch (error) {
      console.error("Failed to fetch headers:", error);
      return [];
    }
  };

  const getAllPrameters = async () => {
    try {
      const parameters = await invoke("get_http_parameters");
      return parameters;
    } catch (error) {
      console.error("Failed to fetch parameters:", error);
      return [];
    }
  };
  return { getMethodColor, getAllMethods, getAllHeaders, getAllPrameters };
}
