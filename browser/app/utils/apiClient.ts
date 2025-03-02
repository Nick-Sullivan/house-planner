import { Configuration, HouseApi, MapApi } from "~/client";

const apiUrl = import.meta.env.VITE_API_URL;
const config = new Configuration({ basePath: apiUrl });

export const houseApi = new HouseApi(config);
export const mapApi = new MapApi(config);
