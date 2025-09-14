import { JsonError } from './error';

export const BASE_PATH = 'api';

// TODO to be tested
export const getMany = <T>(path: string, mapper: (data: any) => T[]): Promise<T[]> => {
  return fetch(`${BASE_PATH}/${path}`, {
    method: 'GET',
  })
    .then(response => {
      return response.json();
    })
    .then(data => {
      return mapper(data);
    });
}


// TODO to be tested
export const getOne = <T>(path: string, mapper: (data: any) => T): Promise<T> => {
  return fetch(`${BASE_PATH}/${path}`, {
  })
    .then((response: Response) => {
      if (response.status === 403) {
        throw new JsonError({ code: 403, message: 'Forbidden' });
      }

      return response.json();
    })
    .then(data => {
      return mapper(data);
    });
}

// TODO to be tested
export const post = <R, T>(path: string, body: R, mapper: (data: any) => T): Promise<T> => {
  return fetch(`${BASE_PATH}/${path}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  })
    .then(response => {
      return response.json();
    })
    .then(data => {
      return mapper(data);
    });
}