import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { CreateUserModule } from './create-user/create-user.module';
import { HttpService } from 'src/services/http.service';
import { UserService } from 'src/services/user.service';
import { HttpClientModule } from '@angular/common/http';

@NgModule({
	declarations: [AppComponent],
	imports: [
		BrowserModule,
		AppRoutingModule,
		CreateUserModule,
		ReactiveFormsModule,
		HttpClientModule,
	],
	providers: [HttpService, UserService],
	bootstrap: [AppComponent],
})
export class AppModule {}
