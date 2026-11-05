import axios, { InternalAxiosRequestConfig } from 'axios';

/* Create an axios instance with the default configuration
 * **Note**: This function is used to create the initial instance of axios, it's used in development mode
 */
export function createInitialInstance() {
  return axios.create({
    baseURL: 'https://beta.appflowy.cloud',
    headers: {
      'Content-Type': 'application/json',
    },
  });
}

export function getAccessToken() {
  const token = localStorage.getItem('token');

  if(!token) {
    return null;
  }

  const parsedToken = JSON.parse(token);

  return parsedToken.access_token;
}

export function requestInterceptor(config: InternalAxiosRequestConfig) {

  const access_token = getAccessToken();

  if(access_token) {
    Object.assign(config.headers, {
      Authorization: `Bearer ${access_token}`,
    });
  }

  return config;
}

export function readableStreamToAsyncIterator(reader: ReadableStreamDefaultReader<Uint8Array>) {
  return {
    async * [Symbol.asyncIterator]() {
      try {
        while(true) {
          const { done, value } = await reader.read();

          if(done) return;
          yield value;
        }
      } finally {
        reader.releaseLock();
      }
    },
  };
}