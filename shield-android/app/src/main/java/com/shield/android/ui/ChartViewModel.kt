package com.shield.android.ui

import androidx.lifecycle.ViewModel

class ChartViewModel : ViewModel() {

    // Simulated data: [Weak, Medium, Strong, Very Strong]
    fun getPasswordStrengthData(): List<Int> {
        return listOf(5, 12, 25, 8)
    }

    // Simulated data: Type -> Count
    fun getAccountTypeData(): Map<String, Int> {
        return mapOf(
            "Social" to 15,
            "Finance" to 5,
            "Work" to 8,
            "Entertainment" to 12,
            "Other" to 10
        )
    }
}
