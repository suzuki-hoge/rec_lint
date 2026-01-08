<?php

namespace App\Service;

class UserService
{
    public function getUser(int $id): string
    {
        return "user-$id";
    }
}
