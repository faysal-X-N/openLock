package com.shield.android.ui

import android.graphics.Color
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.ViewModelProvider
import com.github.mikephil.charting.components.Description
import com.github.mikephil.charting.data.BarData
import com.github.mikephil.charting.data.BarDataSet
import com.github.mikephil.charting.data.BarEntry
import com.github.mikephil.charting.data.PieData
import com.github.mikephil.charting.data.PieDataSet
import com.github.mikephil.charting.data.PieEntry
import com.github.mikephil.charting.utils.ColorTemplate
import com.shield.android.databinding.ActivityDashboardBinding

class DashboardActivity : AppCompatActivity() {
    private lateinit var binding: ActivityDashboardBinding
    private lateinit var viewModel: ChartViewModel

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityDashboardBinding.inflate(layoutInflater)
        setContentView(binding.root)

        viewModel = ViewModelProvider(this)[ChartViewModel::class.java]

        setupBarChart()
        setupPieChart()
    }

    private fun setupBarChart() {
        val strengthData = viewModel.getPasswordStrengthData()
        val entries = strengthData.mapIndexed { index, count ->
            BarEntry(index.toFloat(), count.toFloat())
        }

        val dataSet = BarDataSet(entries, "Password Strength")
        dataSet.colors = ColorTemplate.MATERIAL_COLORS.toList()
        
        val barData = BarData(dataSet)
        binding.chartStrength.data = barData
        binding.chartStrength.description = Description().apply { text = "" }
        binding.chartStrength.animateY(1000)
        binding.chartStrength.invalidate()
    }

    private fun setupPieChart() {
        val typeData = viewModel.getAccountTypeData()
        val entries = typeData.map { (type, count) ->
            PieEntry(count.toFloat(), type)
        }

        val dataSet = PieDataSet(entries, "Account Types")
        dataSet.colors = ColorTemplate.JOYFUL_COLORS.toList()
        dataSet.valueTextSize = 12f
        dataSet.valueTextColor = Color.WHITE

        val pieData = PieData(dataSet)
        binding.chartTypes.data = pieData
        binding.chartTypes.description = Description().apply { text = "" }
        binding.chartTypes.centerText = "Total: ${typeData.values.sum()}"
        binding.chartTypes.animateXY(1000, 1000)
        binding.chartTypes.invalidate()
    }
}
