// SPDX-License-Identifier: MIT
pragma solidity ^0.8.11;
import "../lib/oz/contracts//token/ERC20/IERC20.sol";

/**
 * @title Rescue
 * @notice Simple interface to help rescue assets in case of private key theft
 * @author Verum, John, Lyron
 */
contract Rescue {
    /************************************************
     *  STORAGE
    ***********************************************/

    /************************************************
     *  IMMUTABLES & CONSTANTS
    ***********************************************/

    /************************************************
     *  EVENTS, ERRORS, MODIFIERS
    ***********************************************/
    /// Emit when an asset is transfered in the rescueAssets function
    event AssetTransfer(address indexed from, address to, address amount);

    /// Emit when an asset fails to transfer in the rescueAssets function
    event AssetTransferFailure(address indexed from, address erc20Contract, uint256 amount);

    /**
     * @notice Rescue function 
     * @dev approve() should be called on each ERC20 asset prior to calling this function
     * @param erc20Addresses array of ERC20 assets that should be transfered to the backup Address
     * @param backupAddress the backup address to send funds too
    */
    function rescueAssets(address[] calldata erc20Addresses, address backupAddress) public {
        // Loop through each ERC20 token and move them to the backupAddress
        for (uint i = 0; i < erc20Addresses.length; i++) {
            uint256 tokenBalance = IERC20(erc20Addresses[i]).balanceOf(msg.sender);
            // Function call will throw in event of failure as per ERC20 spec
            try IERC20(erc20Addresses[i]).transferFrom(msg.sender, backupAddress, tokenBalance) {
                // nothing to do
            } catch {
                // Simply log that this asset couldn't be transferred to the backup address
                emit AssetTransferFailure(msg.sender, erc20Addresses[i], tokenBalance);
            }
        }
    }
}