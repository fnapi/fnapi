/** @type {import('@jest/types').Config.InitialOptions} */
export default {
  transform: {
    "^.+\\.(t|j)sx?$": "@swc/jest",
  },
};
