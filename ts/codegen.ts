import codegen from "@cosmwasm/ts-codegen";

codegen({
  contracts: [
    {
      name: "WrapprFactory",
      dir: "../contracts/wrappr-factory/schema",
    },
    {
      name: "WrapprMinter",
      dir: "../contracts/wrappr-minter/schema",
    }
    // {
    //   name: "CwWrappr",
    //   dir: "../contracts/cw-wrappr/schema",
    // },
  
  ],
  outPath: "./src/",

  // options are completely optional ;)
  options: {
    bundle: {
      bundleFile: "index.ts",
      scope: "contracts",
    },
    types: {
      enabled: true,
    },
    client: {
      enabled: true,
    },
    reactQuery: {
      enabled: false,
      optionalClient: true,
      version: "v4",
      mutations: true,
      queryKeys: true,
    },
    recoil: {
      enabled: false,
    },
    messageComposer: {
      enabled: true,
    },
  },
}).then(() => {
  console.log("✨ all done!");
});
