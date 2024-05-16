from django.contrib import admin
from crud.models import Test

# Register your models here.
@admin.register(Test)
class Test(admin.ModelAdmin):
    pass