// SPDX-License-Identifier: (MIT or Apache-2.0)

pragma solidity ^0.8.19;

contract RecentParticipants {
    address[][] public daysRing;
    address private owner;
    
    constructor (address _owner) {
        owner = _owner;
        daysRing = new address[][](3);
    }

    function set (uint date, address[] calldata participants) public {
        require(msg.sender == owner, "Unauthorized");
        daysRing[date % daysRing.length] = participants;
    }

    function get () public view returns (address[] memory) {
        uint participantCount = 0;
        for (uint i = 0; i < daysRing.length; i++) {
            participantCount += daysRing[i].length;
        }
        address[] memory participants = new address[](participantCount);
        uint participantsIndex = 0;
        for (uint i = 0; i < daysRing.length; i++) {
            for (uint j = 0; j < daysRing[i].length; j++) {
                participants[participantsIndex++] = daysRing[i][j];
            }
        }
        return participants;
    }
}
