export const prefixSlash = (input: string) => {
  return input.startsWith("/") ? input : "/" + input;
};
