/** @type {import('ts-jest/dist/types').InitialOptionsTsJest} */
module.exports = {
  testEnvironment: "node",
  extensionsToTreatAsEsm: [".ts", ".tsx"],
  transformIgnorePatterns: ["/node_modules/"],
  transform: {
    "^.+\\.(t|j)sx?$": ["@swc/jest"],
  },
};
