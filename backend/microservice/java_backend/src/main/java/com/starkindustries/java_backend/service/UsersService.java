package com.starkindustries.java_backend.service;

import java.util.Arrays;
import java.util.List;
import org.springframework.stereotype.Service;
import com.starkindustries.java_backend.model.Users;

@Service
public class UsersService {

    
    public List<Users> getDummyUsers() {

        return Arrays.asList(
            Users.builder()
                .userId("1")
                .name("John Doe")
                .email("john.doe@example.com")
                .contact("+1234567890")
                .username("johndoe")
                .password("pass123")
                .build(),
            Users.builder()
                .userId("2")
                .name("Jane Smith")
                .email("jane.smith@example.com")
                .contact("+1987654321")
                .username("janesmith")
                .password("secure456")
                .build(),
            Users.builder()
                .userId("3")
                .name("Alice Johnson")
                .email("alice.j@example.com")
                .contact("+1122334455")
                .username("alicej")
                .password("alice789")
                .build(),
            Users.builder()
                .userId("4")
                .name("Bob Wilson")
                .email("bob.wilson@example.com")
                .contact("+1555666777")
                .username("bobwilson")
                .password("bob101")
                .build(),
            Users.builder()
                .userId("5")
                .name("Charlie Brown")
                .email("charlie.brown@example.com")
                .contact("+1888999000")
                .username("charlieb")
                .password("brown202")
                .build(),
            Users.builder()
                .userId("6")
                .name("Diana Prince")
                .email("diana.prince@example.com")
                .contact("+1333444555")
                .username("dianap")
                .password("wonder303")
                .build(),
            Users.builder()
                .userId("7")
                .name("Eve Davis")
                .email("eve.davis@example.com")
                .contact("+1666777888")
                .username("evedavis")
                .password("eve404")
                .build(),
            Users.builder()
                .userId("8")
                .name("Frank Miller")
                .email("frank.miller@example.com")
                .contact("+1999000011")
                .username("frankm")
                .password("frank505")
                .build(),
            Users.builder()
                .userId("9")
                .name("Grace Hopper")
                .email("grace.hopper@example.com")
                .contact("+1223344556")
                .username("graceh")
                .password("hopper606")
                .build(),
            Users.builder()
                .userId("10")
                .name("Henry Ford")
                .email("henry.ford@example.com")
                .contact("+1777888999")
                .username("henryf")
                .password("ford707")
                .build()
        );
    }
}

