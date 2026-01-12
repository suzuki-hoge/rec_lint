package com.example.entity

data class User(
    private val name: String,
    private val age: Int
) {
    private fun getName(): String = name
    internal fun getAge(): Int = age
}
