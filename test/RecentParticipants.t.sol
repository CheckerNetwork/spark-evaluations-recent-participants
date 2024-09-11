// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/RecentParticipants.sol";

contract RecentParticipantsTest is Test {
    function test_Set() public {
        RecentParticipants rp = new RecentParticipants(address(this));

        // Set initial values
        address[] memory participants = new address[](3);
        participants[0] = address(0x1);
        participants[1] = address(0x2);
        participants[2] = address(0x3);
        rp.set(0, participants);

        assertEq(rp.daysRing(0, 0), address(0x1));
        assertEq(rp.daysRing(0, 1), address(0x2));
        assertEq(rp.daysRing(0, 2), address(0x3));
        vm.expectRevert();
        rp.daysRing(1, 0);
        vm.expectRevert();
        rp.daysRing(2, 0);

        // Add new values and test ring
        participants[0] = address(0x4);
        participants[1] = address(0x5);
        participants[2] = address(0x6);
        rp.set(4, participants);
        assertEq(rp.daysRing(0, 0), address(0x1));
        assertEq(rp.daysRing(0, 1), address(0x2));
        assertEq(rp.daysRing(0, 2), address(0x3));
        assertEq(rp.daysRing(1, 0), address(0x4));
        assertEq(rp.daysRing(1, 1), address(0x5));
        assertEq(rp.daysRing(1, 2), address(0x6));
        vm.expectRevert();
        rp.daysRing(2, 0);

        // Overwrite values
        participants[0] = address(0x7);
        participants[1] = address(0x8);
        participants[2] = address(0x9);
        rp.set(4, participants);
        assertEq(rp.daysRing(0, 0), address(0x1));
        assertEq(rp.daysRing(0, 1), address(0x2));
        assertEq(rp.daysRing(0, 2), address(0x3));
        assertEq(rp.daysRing(1, 0), address(0x7));
        assertEq(rp.daysRing(1, 1), address(0x8));
        assertEq(rp.daysRing(1, 2), address(0x9));
        vm.expectRevert();
        rp.daysRing(2, 0);

        // Auth
        RecentParticipants rp2 = new RecentParticipants(address(0x1));
        vm.expectRevert("Unauthorized");
        rp2.set(0, participants);
    }

    function test_Get() public {
        RecentParticipants rp = new RecentParticipants(address(this));
        address[] memory participants = new address[](3);
        participants[0] = address(0x1);
        participants[1] = address(0x2);
        participants[2] = address(0x3);
        rp.set(0, participants);
        participants[0] = address(0x4);
        participants[1] = address(0x5);
        participants[2] = address(0x6);
        rp.set(4, participants);
        address[] memory allParticipants = rp.get();
        assertEq(allParticipants.length, 6);
        assertEq(allParticipants[0], address(0x1));
        assertEq(allParticipants[1], address(0x2));
        assertEq(allParticipants[2], address(0x3));
        assertEq(allParticipants[3], address(0x4));
        assertEq(allParticipants[4], address(0x5));
        assertEq(allParticipants[5], address(0x6));
    }
}
