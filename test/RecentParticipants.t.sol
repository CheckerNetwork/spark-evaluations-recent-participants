// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/RecentParticipants.sol";

contract RecentParticipantsTest is Test {
    function test_Set() public {
        RecentParticipants rp = new RecentParticipants(address(this));

        // Set initial values
        uint[] memory ids = new uint[](3);
        ids[0] = 1;
        ids[1] = 2;
        ids[2] = 3;
        rp.set(0, ids);

        assertEq(rp.daysRing(0, 0), 1);
        assertEq(rp.daysRing(0, 1), 2);
        assertEq(rp.daysRing(0, 2), 3);
        vm.expectRevert();
        rp.daysRing(1, 0);
        vm.expectRevert();
        rp.daysRing(2, 0);

        // Add new values and test ring
        ids[0] = 4;
        ids[1] = 5;
        ids[2] = 6;
        rp.set(4, ids);
        assertEq(rp.daysRing(0, 0), 1);
        assertEq(rp.daysRing(0, 1), 2);
        assertEq(rp.daysRing(0, 2), 3);
        assertEq(rp.daysRing(1, 0), 4);
        assertEq(rp.daysRing(1, 1), 5);
        assertEq(rp.daysRing(1, 2), 6);
        vm.expectRevert();
        rp.daysRing(2, 0);

        // Overwrite values
        ids[0] = 7;
        ids[1] = 8;
        ids[2] = 9;
        rp.set(4, ids);
        assertEq(rp.daysRing(0, 0), 1);
        assertEq(rp.daysRing(0, 1), 2);
        assertEq(rp.daysRing(0, 2), 3);
        assertEq(rp.daysRing(1, 0), 7);
        assertEq(rp.daysRing(1, 1), 8);
        assertEq(rp.daysRing(1, 2), 9);
        vm.expectRevert();
        rp.daysRing(2, 0);

        // Auth
        RecentParticipants rp2 = new RecentParticipants(address(0x1));
        vm.expectRevert("Unauthorized");
        rp2.set(0, ids);
    }

    function test_Get() public {
        RecentParticipants rp = new RecentParticipants(address(this));
        uint[] memory ids = new uint[](3);
        ids[0] = 1;
        ids[1] = 2;
        ids[2] = 3;
        rp.set(0, ids);
        ids[0] = 4;
        ids[1] = 5;
        ids[2] = 6;
        rp.set(4, ids);
        uint[] memory participants = rp.get();
        assertEq(participants.length, 6);
        assertEq(participants[0], 1);
        assertEq(participants[1], 2);
        assertEq(participants[2], 3);
        assertEq(participants[3], 4);
        assertEq(participants[4], 5);
        assertEq(participants[5], 6);
    }
}
