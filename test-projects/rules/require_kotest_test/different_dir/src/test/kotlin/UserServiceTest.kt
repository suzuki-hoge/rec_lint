package com.example.service

import io.kotest.core.spec.style.StringSpec

class UserServiceTest : StringSpec({
    "getUser returns user with id" {
        val service = UserService()
        assert(service.getUser(1) == "user-1")
    }
})
