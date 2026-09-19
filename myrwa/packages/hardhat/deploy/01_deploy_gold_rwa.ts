import { deployScript, artifacts } from "../rocketh/deploy.js";

export default deployScript(
  async ({ deploy, namedAccounts }) => {
    const { deployer: admin } = namedAccounts;

    const assetDocument =
      "ipfs://QmExampleGoldReserveReport2026";

    const result = await deploy("GoldRWAToken", {
      account: admin,
      artifact: artifacts.GoldRWAToken,
      args: [
        assetDocument,
        admin,
      ],
    });

    console.log(
      "GoldRWAToken deployed:",
      result.address,
    );
  },
  {
    tags: ["GoldRWAToken"],
  },
);