// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

contract GoldRWAToken is ERC20, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant ASSET_MANAGER_ROLE =
        keccak256("ASSET_MANAGER_ROLE");

    string private _assetDocument;

    event TokensMinted(
        address indexed to,
        uint256 amount,
        address indexed minter
    );

    event TokensBurned(
        address indexed from,
        uint256 amount
    );

    event AssetDocumentUpdated(
        string oldDocument,
        string newDocument,
        address indexed updater
    );

    constructor(
        string memory initialAssetDocument,
        address admin
    ) ERC20("Gold RWA Token", "GLDR") {
        require(admin != address(0), "Invalid admin");

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MINTER_ROLE, admin);
        _grantRole(ASSET_MANAGER_ROLE, admin);

        _assetDocument = initialAssetDocument;
    }

    /**
     * @notice Issue RWA tokens
     * @dev Only accounts with MINTER_ROLE can call this function.
     */
    function mint(
        address to,
        uint256 amount
    ) external onlyRole(MINTER_ROLE) {
        require(to != address(0), "Invalid recipient");
        require(amount > 0, "Amount must be greater than zero");

        _mint(to, amount);

        emit TokensMinted(to, amount, msg.sender);
    }

    /**
     * @notice Burn caller's own tokens.
     */
    function burn(
        uint256 amount
    ) external {
        require(amount > 0, "Amount must be greater than zero");

        _burn(msg.sender, amount);

        emit TokensBurned(msg.sender, amount);
    }

    /**
     * @notice Update the simulated asset proof document.
     */
    function updateAssetDocument(
        string calldata document
    ) external onlyRole(ASSET_MANAGER_ROLE) {
        require(bytes(document).length > 0, "Empty document");

        string memory oldDocument = _assetDocument;

        _assetDocument = document;

        emit AssetDocumentUpdated(
            oldDocument,
            document,
            msg.sender
        );
    }

    /**
     * @notice Return current asset proof document.
     */
    function assetDocument()
        external
        view
        returns (string memory)
    {
        return _assetDocument;
    }
}