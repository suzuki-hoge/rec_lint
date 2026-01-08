<?php

class 一部メソッド未テストTest extends TestCase
{
    public function testCreateUser()
    {
        $service = new 一部メソッド未テスト();
        $service->createUser();
    }
    // 一部のメソッドはテストされていない
}
