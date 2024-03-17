import codegen from "@cosmwasm/ts-codegen";

codegen({
  contracts: [
    {
      name: "WrapprFactory",
      dir: "../contracts/factories/wrappr-factory/schema",
    },
    {
      name: "WrapprMinter",
      dir: "../contracts/minters/wrappr-minter/schema",
    },
    {
      name: "Wrappr721Base",
      dir: "../contracts/collections/wrappr721-base/schema",
    },
    // {
    //   name: "Sg721MetadataOnchain",
    //   dir: "../contracts/collections/wrappr721-metadata-onchain/schema",
    // },
    // {
    //   name: "Sg721Updatable",
    //   dir: "../contracts/collections/wrappr721-updatable/schema",
    // },
    // {
    //   name: "Sg721Nt",
    //   dir: "../contracts/collections/wrappr721-nt/schema",
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
