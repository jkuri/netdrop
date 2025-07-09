function c<T>(prod: T, dev: T): T {
  return import.meta.env.DEV ? dev : prod;
}

export const config = {
  BASE_URL: c(window.location.origin, "http://localhost:8000"),
};
