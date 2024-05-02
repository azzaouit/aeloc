// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.7;

contract AelocDispatcher {
     event BoundingBox (
        int256 xmin,
        int256 ymin,
        int256 xmax,
        int256 ymax,
        string key,
        string val,
        int256 limit,
        address caller
    );

    event Geocode(
        string location,
        address caller
    );

    event ReverseGeocode(
        int256 lat,
        int256 lon,
        address caller
    );

    function _bounding_box(
        int256 xmin,
        int256 ymin,
        int256 xmax,
        int256 ymax,
        string memory key,
        string memory val,
        int256 limit
    ) public virtual {
        emit BoundingBox(xmin, ymin, xmax, ymax, key, val, limit, address(msg.sender));
    }

    function _geocode(string memory location) public virtual {
        emit Geocode(location, address(msg.sender));
    }

    function _reverse_geocode(int256 lat, int256 lon) public virtual {
        emit ReverseGeocode(lat, lon, address(msg.sender));
    }

    function _geocode_callback(uint256 node) public virtual {}

    function _reverse_geocode_callback(uint256 node) public virtual {}

    function _bounding_box_callback(uint256[] calldata nodes) public virtual {}
}
