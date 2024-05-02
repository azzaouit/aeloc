// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.7;

import {AelocDispatcher} from "./AelocDispatcher.sol";
import {IAelocDispatcher} from "./IAelocDispatcher.sol";

contract AelocRunner is AelocDispatcher {
    address _dispatcher;

    constructor(address dispatcher){
        _dispatcher = dispatcher;
    }

    function _bounding_box(
        int256 xmin,
        int256 ymin,
        int256 xmax,
        int256 ymax,
        string memory key,
        string memory val,
        int256 limit
    ) public override {
        return IAelocDispatcher(_dispatcher)._bounding_box(xmin, ymin, xmax, ymax, key, val, limit);
    }

    function _geocode(string memory location) public override {
        return IAelocDispatcher(_dispatcher)._geocode(location);
    }

    function _reverse_geocode(int256 lat, int256 lon) public override {
        return IAelocDispatcher(_dispatcher)._reverse_geocode(lat, lon);
    }
}
