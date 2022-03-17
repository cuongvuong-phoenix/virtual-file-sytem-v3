import _axios from 'axios';

export const axios = _axios.create({
  baseURL: import.meta.env.VITE_SERVER_ADDRESS,
});
