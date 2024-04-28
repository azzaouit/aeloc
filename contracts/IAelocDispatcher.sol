// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

interface IAelocDispatcher {
    function _bounding_box(
        int256 xmin,
        int256 ymin,
        int256 xmax,
        int256 ymax,
        string memory key,
        string memory val,
        int256 limit
    ) external;

    function _geocode(string memory location) external;

    function _reverse_geocode(int256 lat, int256 lon) external;
}
