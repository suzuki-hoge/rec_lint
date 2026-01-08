<?php

class 全メソッドテスト済みTest extends TestCase
{
    public function testCreateUser()
    {
        $service = new 全メソッドテスト済み();
        $service->createUser();
    }

    public function testDeleteUser()
    {
        $service = new 全メソッドテスト済み();
        $service->deleteUser();
    }
}
