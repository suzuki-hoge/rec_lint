<?php

namespace db;

use PDO;
use Symfony\Component\HttpClient\HttpClient;

/**
 * サンプル
 */
class PlanQuery
{
    public function fetch(PDO $conn): void
    {
        var_dump(42);
    }
}
