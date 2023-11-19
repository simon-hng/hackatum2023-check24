import axios from "axios";
import process from "process";

export const api = axios.create({
  baseURL: process.env.PUBLIC_API_BASE_URL,
});
