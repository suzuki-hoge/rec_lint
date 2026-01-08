<?php
class Test {
    #[Test]
    #[DataProvider('userProvider')]
    public function shouldCreateUser() {}
}
