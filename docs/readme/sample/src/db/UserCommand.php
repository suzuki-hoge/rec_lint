<?php

namespace db;

use PDO;
use Symfony\Component\HttpClient\HttpClient;

/**
 * サンプル
 */
class UserCommand
{
    public function create(PDO $conn): void
    {
        var_dump(42);
    }
}
