import { expect } from "chai";
import hre from "hardhat";

describe("GoldRWAToken", function () {
  async function deployToken() {
    const { ethers } = await hre.network.connect();

    const [admin, user, attacker] =
      await ethers.getSigners();

    const GoldRWAToken =
      await ethers.getContractFactory("GoldRWAToken");

    const token = await GoldRWAToken.deploy(
      "ipfs://QmExampleGoldReserveReport2026",
      admin.address
    );

    await token.waitForDeployment();

    return {
      token,
      admin,
      user,
      attacker,
      ethers,
    };
  }

  it("should deploy successfully", async function () {
    const { token, ethers } = await deployToken();

    expect(await token.getAddress()).to.not.equal(
      ethers.ZeroAddress
    );
  });

  it("should have correct token information", async function () {
    const { token } = await deployToken();

    expect(await token.name()).to.equal(
      "Gold RWA Token"
    );

    expect(await token.symbol()).to.equal(
      "GLDR"
    );

    expect(await token.decimals()).to.equal(18);
  });

  it("authorized account can mint", async function () {
    const { token, admin, user, ethers } =
      await deployToken();

    const amount = ethers.parseEther("10");

    await token
      .connect(admin)
      .mint(user.address, amount);

    expect(
      await token.balanceOf(user.address)
    ).to.equal(amount);

    expect(
      await token.totalSupply()
    ).to.equal(amount);
  });

it("unauthorized account cannot mint", async function () {
  const { token, attacker, user, ethers } =
    await deployToken();

  const amount = ethers.parseEther("10");

  await expect(
    token
      .connect(attacker)
      .mint(user.address, amount)
  ).to.be.revert(ethers);
});
  it("holder can transfer tokens", async function () {
    const { token, admin, user, attacker, ethers } =
      await deployToken();

    const amount = ethers.parseEther("10");
    const transferAmount = ethers.parseEther("3");

    await token
      .connect(admin)
      .mint(user.address, amount);

    await token
      .connect(user)
      .transfer(
        attacker.address,
        transferAmount
      );

    expect(
      await token.balanceOf(attacker.address)
    ).to.equal(transferAmount);

    expect(
      await token.balanceOf(user.address)
    ).to.equal(
      amount - transferAmount
    );
  });

  it("holder can burn tokens", async function () {
    const { token, admin, user, ethers } =
      await deployToken();

    const amount = ethers.parseEther("10");
    const burnAmount = ethers.parseEther("4");

    await token
      .connect(admin)
      .mint(user.address, amount);

    await token
      .connect(user)
      .burn(burnAmount);

    expect(
      await token.balanceOf(user.address)
    ).to.equal(
      amount - burnAmount
    );

    expect(
      await token.totalSupply()
    ).to.equal(
      amount - burnAmount
    );
  });

  it(
    "authorized account can update asset document",
    async function () {
      const { token, admin } =
        await deployToken();

      const newDocument =
        "ipfs://QmNewGoldReport";

      await token
        .connect(admin)
        .updateAssetDocument(newDocument);

      expect(
        await token.assetDocument()
      ).to.equal(newDocument);
    }
  );

  it(
  "unauthorized account cannot update asset document",
  async function () {
    const { token, attacker, ethers } =
      await deployToken();

    await expect(
      token
        .connect(attacker)
        .updateAssetDocument(
          "ipfs://QmFakeReport"
        )
    ).to.be.revert(ethers);
  }
);

  it("should have correct roles", async function () {
    const { token, admin } =
      await deployToken();

    const MINTER_ROLE =
      await token.MINTER_ROLE();

    const ASSET_MANAGER_ROLE =
      await token.ASSET_MANAGER_ROLE();

    expect(
      await token.hasRole(
        MINTER_ROLE,
        admin.address
      )
    ).to.equal(true);

    expect(
      await token.hasRole(
        ASSET_MANAGER_ROLE,
        admin.address
      )
    ).to.equal(true);
  });
});