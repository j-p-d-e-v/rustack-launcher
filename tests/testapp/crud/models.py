from django.db import models

# Create your models here.
class Test(models.Model):

    name = models.CharField(verbose_name="Name",max_length=300,null=True,blank=True)

    class Meta:
        verbose_name = "Test"
        verbose_name_plural = "Tests"
    
    def __str__(self):
        return self.name