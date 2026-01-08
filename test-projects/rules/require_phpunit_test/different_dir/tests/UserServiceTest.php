<?php

namespace App\Service;

use PHPUnit\Framework\TestCase;

class UserServiceTest extends TestCase
{
    public function testGetUser(): void
    {
        $service = new UserService();
        $this->assertEquals("user-1", $service->getUser(1));
    }
}
