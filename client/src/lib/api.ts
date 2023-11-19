import axios from "axios";
import { env } from "~/env.mjs";

export const api = axios.create({
  baseURL: env.NEXT_PUBLIC_API_BASE_URL,
});

export const proxyApi = axios.create({
  baseURL: '/'
});
