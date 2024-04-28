# Aeloc

A spatial data oracle for the EVM

## Project Status

Aeloc is live on the Sepolia testnet. Open an [issue](https://github.com/azzaouit/aeloc/issues) to report a bug or request a feature.

- dispatcher: 0x775ca67487BD218df9f520BBA933536cE4F36a5b

## Data Feeds

- [x] Geocoding
- [x]  Reverse Geocoding

## What's Aeloc?

Aeloc is an oracle tailored for processing spatial data. It consists of
* A web service for querying map data.
* A codec library for encoding geographic data structures into EVM types.
* An oracle service for broadcasting data on chain.
* A smart contract library for accessing map data from any EVM chain.

## Why build another oracle?

General purpose oracles are flexible with the data formats they provide.
Aeloc exploits the structure of geographic data to speed up processing and lower tx fees.

# Examples

## Usage

Compile and deploy with the dispatcher address on the Sepolia testnet.
Call the contract's `fill_node()` function.
The oracle will invoke the callback function with a response.

## Geocode

Geocoding maps a description like "New York" or "London" to a point in space.
Use this feed to convert an address into a coordinate pair.
        
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import {AelocRunner} from "@aeloc/contracts/AelocRunner.sol";

contract Runner is AelocRunner {
    uint256 public _geocode_node;

    constructor(address dispatcher) AelocRunner(dispatcher){}

    function fill_node() public {
        _geocode("221B Baker St, London");
    }

    function _geocode_callback(uint256 node) public override {
        _geocode_node = node;
    }
}
```

## Reverse Geocode

 Reverse geocoding maps a coordinate pair to a description like "New York" or "London".
 Use this feed if you already know the coordinates of your location.
 Note that all decimal values are scaled by $10^8$ and represented as integers.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import {AelocRunner} from "@aeloc/contracts/AelocRunner.sol";

contract Runner is AelocRunner {
    uint256 public _reverse_geocode_node;

    constructor(address dispatcher) AelocRunner(dispatcher){}

    function fill_node() public {
        _reverse_geocode(5152338790, -15823670);
    }

    function _reverse_geocode_callback(uint256 node) public override {
        _reverse_geocode_node = node;
    }
}
```

## Bounding Boxes

Search for points within a bounding box. 
Use [bboxfinder](http://bboxfinder.com/#40.776620,-73.988400,40.805000,-73.972300) to draw bounding boxes.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import {AelocRunner} from "@aeloc/contracts/AelocRunner.sol";

contract Runner is AelocRunner {
    uint256[5] public _bounding_box_nodes;

    constructor(address dispatcher) AelocRunner(dispatcher){}

    function fill_nodes() public {
        _bounding_box(
            4077662222, /* xmin */
            -7398845555, /* ymin */
            4080505555, /* xmax */
            -7397236944, /* ymax */
            "amenity", /* key */
            "cafe", /* val */
            5 /* limit */
        );

    }

    function _bounding_box_callback(uint256[] calldata nodes) public override {
        for(uint i = 0; i < 5; i++){
            _bounding_box_nodes[i] = nodes[i];
        }
    }
}
```
