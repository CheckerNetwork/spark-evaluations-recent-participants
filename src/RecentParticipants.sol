// SPDX-License-Identifier: (MIT or Apache-2.0)

pragma solidity ^0.8.19;

contract RecentParticipants {
    uint[][] public daysRing;
    address private owner;
    
    constructor (address _owner) {
        owner = _owner;
        daysRing = new uint[][](3);
    }

    function set (uint date, uint[] calldata participants) public {
        require(msg.sender == owner, "Unauthorized");
        daysRing[date % daysRing.length] = participants;
    }

    function get () public view returns (uint[] memory) {
        uint participantCount = 0;
        for (uint i = 0; i < daysRing.length; i++) {
            participantCount += daysRing[i].length;
        }
        uint[] memory participants = new uint[](participantCount);
        uint participantsIndex = 0;
        for (uint i = 0; i < daysRing.length; i++) {
            for (uint j = 0; j < daysRing[i].length; j++) {
                participants[participantsIndex++] = daysRing[i][j];
            }
        }
        return participants;
    }
}
