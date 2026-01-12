<?php

namespace App\Entity;

class User
{
    private string $name;
    private int $age;

    private function getName(): string
    {
        return $this->name;
    }

    protected function getAge(): int
    {
        return $this->age;
    }
}
